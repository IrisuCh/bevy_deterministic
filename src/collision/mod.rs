#![allow(clippy::missing_panics_doc)]

mod aabb;
mod substep;
pub mod tilemap_backend;

use crate::collision::aabb::Aabb;
use crate::collision::substep::SubstepIterator;
use crate::determinism::transform::{GlobalPosition, Position, Size};
use crate::physics::KinematicRigidBody;
use bevy::prelude::*;
use fixed::types::I32F32;
use std::collections::{HashMap, HashSet};

#[derive(EntityEvent)]
pub struct CollisionEnter {
    pub entity: Entity,
    pub side: CollisionSide,
    pub offset: I32F32,
}

#[derive(EntityEvent)]
pub struct Collision {
    pub entity: Entity,
    pub side: CollisionSide,
    pub offset: I32F32,
}

#[derive(EntityEvent)]
pub struct CollisionExit {
    pub entity: Entity,
    pub other: Entity,
}

#[derive(Component, Reflect, Debug, Default)]
#[require(Position)]
pub struct Collider {
    pub(crate) enter: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CollisionSide {
    Left,   // -X
    Right,  // +X
    Bottom, // -Y (в играх часто низ — это Bottom или Down)
    Top,    // +Y (верх — Top или Up)
    Front,  // +Z (ближе к камере)
    Back,   // -Z (дальше от камеры)
}

fn get_side_and_offset(a: &Aabb, b: &Aabb) -> (CollisionSide, I32F32) {
    // 1. Считаем перекрытия
    let overlap_left = b.max.x - a.min.x;
    let overlap_right = a.max.x - b.min.x;
    let overlap_bottom = b.max.y - a.min.y;
    let overlap_top = a.max.y - b.min.y;
    let overlap_back = b.max.z - a.min.z;
    let overlap_front = a.max.z - b.min.z;

    // 2. Важный момент: если мы в 2D, то разница по Z может быть 0 или некорректна.
    // Чтобы Z не перебивала X и Y, мы проверяем, есть ли вообще объем по Z.
    // Если d() == 0, ставим очень большое число, чтобы min() его не выбрал.
    let dz = if a.d() > 0 && b.d() > 0 {
        overlap_back.min(overlap_front)
    } else {
        I32F32::MAX // Игнорируем Z в 2D
    };

    // 3. Находим фактический минимум среди активных осей
    let min_overlap = overlap_left
        .min(overlap_right)
        .min(overlap_top)
        .min(overlap_bottom)
        .min(dz);

    const CORNER_THRESHOLD: I32F32 = I32F32::const_from_int(2);

    // 4. Приоритетность выбора стороны
    // Если min_overlap совпадает с Z-осью (и мы в 3D), возвращаем Front/Back
    if min_overlap == overlap_front && dz != I32F32::MAX {
        (CollisionSide::Front, overlap_front)
    } else if min_overlap == overlap_back && dz != I32F32::MAX {
        (CollisionSide::Back, overlap_back)
    }
    // Дальше ваша стандартная 2D логика
    else if min_overlap == overlap_left && overlap_bottom < CORNER_THRESHOLD {
        (CollisionSide::Bottom, overlap_bottom)
    } else if min_overlap == overlap_right && overlap_bottom < CORNER_THRESHOLD {
        (CollisionSide::Bottom, overlap_bottom)
    } else if min_overlap == overlap_left {
        (CollisionSide::Left, overlap_left)
    } else if min_overlap == overlap_right {
        (CollisionSide::Right, overlap_right)
    } else if min_overlap == overlap_top {
        (CollisionSide::Top, overlap_top)
    } else {
        (CollisionSide::Bottom, overlap_bottom)
    }
}

pub fn apply_physics(
    mut commands: Commands,
    transform: Query<(Entity, &GlobalPosition, &Size), With<Collider>>,
    dynamic_rigid_body: Query<(Entity, &mut KinematicRigidBody, &mut Collider)>,
    mut overlap_history: Local<HashMap<Entity, HashSet<Entity>>>,
    mut positions: Query<&mut Position>,
) {
    for (entity, mut rigid_body, mut collider) in dynamic_rigid_body {
        let (_, position, size) = transform.get(entity).unwrap();
        let mut iter = SubstepIterator::new(
            position,
            size,
            rigid_body.velocity.x,
            rigid_body.velocity.y,
            rigid_body.velocity.z,
        );

        for (other, other_position, other_size) in transform {
            if entity == other {
                continue;
            }

            let other_rect = Aabb::from_pos_size(other_position.as_position(), other_size);
            let Some(rect) = iter.next_overlap(&other_rect) else {
                let entry = overlap_history.entry(entity).or_default();
                trigger_exit(entity, other, &mut commands, entry);
                if entry.is_empty() {
                    collider.enter = false;
                    overlap_history.remove(&entity);
                }
                continue;
            };

            let (side, offset) = get_side_and_offset(&rect, &other_rect);
            let position = &mut positions.get_mut(entity).unwrap();
            if collider.enter {
                trigger_stay(
                    entity,
                    side,
                    offset,
                    &mut commands,
                    &mut rigid_body,
                    position,
                );
            } else {
                overlap_history.entry(entity).or_default().insert(other);
                collider.enter = true;
                trigger_enter(
                    entity,
                    side,
                    offset,
                    &mut commands,
                    position,
                    &rect,
                    &other_rect,
                );

                trigger_stay(
                    entity,
                    side,
                    offset,
                    &mut commands,
                    &mut rigid_body,
                    position,
                );
            }
        }
    }
}

fn trigger_enter(
    entity: Entity,
    side: CollisionSide,
    offset: I32F32,
    commands: &mut Commands,
    position: &mut Position,
    rect: &Aabb,
    other_rect: &Aabb,
) {
    commands.trigger(CollisionEnter {
        entity,
        side,
        offset,
    });

    //println!(
    //    "Side: {side:?}
    //    Other Rect: {:?}
    //    Player: {:?}
    //    Offset: {:?}
    //    ",
    //    other_rect.x() + other_rect.w(),
    //    rect.x(),
    //    offset,
    //);

    match side {
        CollisionSide::Left => position.x = other_rect.max.x,
        CollisionSide::Right => position.x = other_rect.min.x - rect.w(),

        CollisionSide::Bottom => position.y = other_rect.max.y,
        CollisionSide::Top => position.y = other_rect.min.y - rect.h(),

        CollisionSide::Front => position.z = other_rect.max.z,
        CollisionSide::Back => position.z = other_rect.min.z - rect.d(),
    }
}

fn trigger_stay(
    entity: Entity,
    side: CollisionSide,
    offset: I32F32,
    commands: &mut Commands,
    rigid_body: &mut KinematicRigidBody,
    position: &mut Position,
) {
    commands.trigger(Collision {
        entity,
        side,
        offset,
    });

    match side {
        CollisionSide::Left => rigid_body.velocity.clamp_positive_x(),
        CollisionSide::Right => rigid_body.velocity.clamp_negative_x(),

        CollisionSide::Top => rigid_body.velocity.clamp_negative_y(),
        CollisionSide::Bottom => rigid_body.velocity.clamp_positive_y(),

        CollisionSide::Front => rigid_body.velocity.clamp_negative_z(),
        CollisionSide::Back => rigid_body.velocity.clamp_positive_z(),
    }

    match side {
        CollisionSide::Left => position.x += offset,
        CollisionSide::Right => position.x -= offset,

        CollisionSide::Bottom => position.y += offset,
        CollisionSide::Top => position.y -= offset,

        CollisionSide::Back => position.z += offset,
        CollisionSide::Front => position.z -= offset,
    }
    //println!("[{side:?}] Rigidbody velocity: {:?}", rigid_body.velocity);
}

fn trigger_exit(
    entity: Entity,
    other: Entity,
    commands: &mut Commands,
    entry: &mut HashSet<Entity>,
) {
    if entry.remove(&other) {
        commands.trigger(CollisionExit { entity, other });
    }
}

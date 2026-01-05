#![allow(clippy::missing_panics_doc)]

mod aabb;
mod substep;
pub mod tilemap_backend;

use crate::collision::aabb::AABB;
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

#[derive(Copy, Clone, Debug)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}

fn get_side_and_offset(a: &AABB, b: &AABB) -> (CollisionSide, I32F32) {
    let a_min = a.min();
    let a_max = a.max();
    let b_min = b.min();
    let b_max = b.max();

    //println!("A_MIN: {a_min:?}, A_MAX: {a_max:?}, B_MIN: {b_min:?}, B_MAX: {b_max:?}");

    // Горизонтальная глубина
    let overlap_left = b_max.x - a_min.x; // A зашёл в B слева (A справа, B слева)
    let overlap_right = a_max.x - b_min.x; // A зашёл в B справа

    // Вертикальная глубина
    let overlap_bottom = b_max.y - a_min.y; // A зашёл в B снизу
    let overlap_top = a_max.y - b_min.y; // A зашёл в B сверху

    // Выбираем наименьшее пересечение
    let min_overlap = overlap_left
        .min(overlap_right)
        .min(overlap_top)
        .min(overlap_bottom);

    const CORNER_THRESHOLD: I32F32 = I32F32::const_from_int(2); // пикселя

    if min_overlap == overlap_left && overlap_bottom < CORNER_THRESHOLD {
        // Скорее всего это угол снизу-слева, а не чисто левая коллизия
        (CollisionSide::Bottom, overlap_bottom)
    } else if min_overlap == overlap_right && overlap_bottom < CORNER_THRESHOLD {
        // Угол снизу-справа
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
        let mut iter =
            SubstepIterator::new(position, size, rigid_body.velocity.x, rigid_body.velocity.y);

        for (other, other_position, other_size) in transform {
            if entity == other {
                continue;
            }

            let other_rect = AABB::from_pos_size(other_position, other_size);
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
    rect: &AABB,
    other_rect: &AABB,
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
        CollisionSide::Left => position.x = other_rect.max_x(),
        CollisionSide::Right => position.x = other_rect.x() - rect.w(),
        CollisionSide::Top => position.y = other_rect.y() - rect.h(),
        CollisionSide::Bottom => position.y = other_rect.max_y(),
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
    }

    match side {
        CollisionSide::Left => position.x += offset,
        CollisionSide::Right => position.x -= offset,
        CollisionSide::Top => position.y -= offset,
        CollisionSide::Bottom => position.y += offset,
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

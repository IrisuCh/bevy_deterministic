mod aabb;
mod event;
mod side;
mod substep;

use bevy::prelude::*;
use fixed::types::I32F32;

pub use crate::physics::collision::{aabb::Aabb, side::CollisionSide};
use crate::{
    physics::{
        KinematicRigidBody,
        collision::{
            event::{trigger_enter, trigger_exit, trigger_stay},
            substep::SubstepIterator,
        },
    },
    transform::{GlobalPosition, Position, Size},
};

pub mod prelude {
    pub use super::*;
}

#[derive(Component, Reflect, Debug, Default)]
#[require(Position, Size)]
pub struct Collider;

fn get_side_and_offset(a: &Aabb, b: &Aabb) -> (CollisionSide, I32F32) {
    const CORNER_THRESHOLD: I32F32 = I32F32::const_from_int(2);

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

    // 4. Приоритетность выбора стороны
    // Если min_overlap совпадает с Z-осью (и мы в 3D), возвращаем Front/Back
    if min_overlap == overlap_front && dz != I32F32::MAX {
        (CollisionSide::Front, overlap_front)
    } else if min_overlap == overlap_back && dz != I32F32::MAX {
        (CollisionSide::Back, overlap_back)
    } else if (min_overlap == overlap_left || min_overlap == overlap_right)
        && overlap_bottom < CORNER_THRESHOLD
    {
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

pub(crate) fn apply_physics(
    mut commands: Commands,
    transform: Query<(Entity, &GlobalPosition, &Size), With<Collider>>,
    dynamic_rigid_body: Query<(Entity, &mut KinematicRigidBody)>,
    mut positions: Query<&mut Position>,
) {
    for (current, mut rigid_body) in dynamic_rigid_body {
        let (_, position, size) = transform.get(current).unwrap();
        let mut iter = SubstepIterator::new(
            position,
            size,
            rigid_body.velocity.x,
            rigid_body.velocity.y,
            rigid_body.velocity.z,
        );

        for (other, other_position, other_size) in transform {
            if current == other {
                continue;
            }

            let other_rect = Aabb::from_pos_size(other_position.as_position(), other_size);
            let Some(rect) = iter.next_overlap(&other_rect) else {
                if rigid_body.remove_other(other) {
                    trigger_exit(current, other, &mut commands);
                }
                continue;
            };

            let (side, offset) = get_side_and_offset(&rect, &other_rect);
            let position = &mut positions.get_mut(current).unwrap();
            if rigid_body.has_other(other) {
                trigger_stay(
                    current,
                    side,
                    offset,
                    &mut commands,
                    &mut rigid_body,
                    position,
                );
            } else {
                rigid_body.insert_other(other, side);
                trigger_enter(
                    current,
                    side,
                    offset,
                    &mut commands,
                    position,
                    &rect,
                    &other_rect,
                );

                trigger_stay(
                    current,
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

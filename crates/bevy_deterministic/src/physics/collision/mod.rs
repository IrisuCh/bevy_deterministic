mod aabb;
mod event;
mod side;
mod substep;

use std::f32;

use bevy::prelude::*;

pub use crate::physics::collision::{aabb::Aabb, side::CollisionSide};
use crate::{
    Fx, fx,
    physics::{
        KinematicRigidBody,
        collision::{
            aabb::Obb,
            event::{trigger_enter, trigger_exit, trigger_stay},
            substep::SubstepIterator,
        },
    },
    resources::FixedTime,
    transform::{FVec3, FixedGlobalTransform, FixedTransform},
};

pub mod prelude {
    pub use super::*;
}

#[derive(Component, Reflect, Debug, Default)]
#[require(FixedTransform)]
pub struct Collider;

fn normal_to_side(normal: FVec3) -> CollisionSide {
    let abs_normal = FVec3::new(normal.x.abs(), normal.y.abs(), normal.z.abs());

    // Находим самую большую компоненту нормали
    if abs_normal.x > abs_normal.y && abs_normal.x > abs_normal.z {
        // Доминирует X ось
        if normal.x > Fx::ZERO {
            CollisionSide::Right
        } else {
            CollisionSide::Left
        }
    } else if abs_normal.y > abs_normal.z {
        // Доминирует Y ось
        if normal.y > Fx::ZERO {
            CollisionSide::Top
        } else {
            CollisionSide::Bottom
        }
    } else {
        // Доминирует Z ось
        if normal.z > Fx::ZERO {
            CollisionSide::Front
        } else {
            CollisionSide::Back
        }
    }
}

pub(crate) fn apply_physics(
    mut commands: Commands,
    time: Res<FixedTime>,
    transform: Query<(Entity, &FixedGlobalTransform), With<Collider>>,
    dynamic_rigid_body: Query<(Entity, &mut KinematicRigidBody)>,
    mut positions: Query<&mut FixedTransform>,
) {
    for (current, mut rigid_body) in dynamic_rigid_body {
        let (_, global_transform) = transform.get(current).unwrap();
        let mut iter = SubstepIterator::new(
            global_transform.transform(),
            rigid_body.velocity.x * time.delta_time(),
            rigid_body.velocity.y * time.delta_time(),
            rigid_body.velocity.z * time.delta_time(),
        );

        for (other, other_global_transform) in transform {
            if current == other {
                continue;
            }

            let other_rect = Obb::from_transform(
                other_global_transform.position(),
                other_global_transform.size(),
                other_global_transform.rotation(),
            );

            let Some(collision_info) = iter.next_overlap(&other_rect) else {
                if rigid_body.remove_other(other) {
                    trigger_exit(current, other, &mut commands);
                }
                continue;
            };

            let position = &mut positions.get_mut(current).unwrap();
            position.position -= collision_info.normal * (collision_info.depth - fx!(f32::EPSILON));
            block_movement_along_normal(collision_info.normal, &mut rigid_body.velocity);

            let side = normal_to_side(collision_info.normal);
            if rigid_body.has_other(other) {
                trigger_stay(current, side, &mut commands);
            } else {
                rigid_body.insert_other(other, side);
                trigger_enter(current, side, &mut commands);
                trigger_stay(current, side, &mut commands);
            }
        }
    }
}

fn block_movement_along_normal(normal: FVec3, velocity: &mut FVec3) {
    let normal_vel = velocity.dot(normal);

    if normal_vel > Fx::ZERO {
        *velocity -= normal * normal_vel;
    }
}

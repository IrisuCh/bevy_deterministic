mod aabb;
pub mod event;
mod obb;
mod side;
mod substep;

use bevy::prelude::*;

pub use crate::physics::collision::{aabb::Aabb, side::CollisionSide};
use crate::{
    math::{FVec3, Fx, fx},
    physics::{
        KinematicRigidBody,
        collision::{
            event::{trigger_enter, trigger_exit, trigger_stay},
            obb::Obb,
            substep::SubstepIterator,
        },
    },
    resources::FixedTime,
    transform::{FixedGlobalTransform, FixedTransform},
};

pub mod prelude {
    pub use super::*;
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(FixedTransform)]
pub struct Collider {
    pub trigger: bool,
    pub disabled: bool,
    pub center: FVec3,
    pub size: FVec3,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            trigger: false,
            disabled: false,
            center: FVec3::ZERO,
            size: FVec3::ONE,
        }
    }
}

impl Collider {
    #[must_use]
    pub fn trigger() -> Self {
        Self {
            trigger: true,
            disabled: false,
            ..default()
        }
    }

    #[must_use]
    pub fn disabled() -> Self {
        Self {
            trigger: false,
            disabled: true,
            ..default()
        }
    }

    #[must_use]
    pub fn with_size(mut self, size: FVec3) -> Self {
        self.size = size;
        self
    }

    #[must_use]
    pub fn with_center(mut self, center: FVec3) -> Self {
        self.center = center;
        self
    }

    #[must_use]
    pub fn transform(&self, transform: &FixedTransform) -> FixedTransform {
        let mut transform = transform.clone();
        transform.position += self.center;
        transform.size *= self.size;
        transform
    }
}

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
    transform: Query<(Entity, &FixedGlobalTransform, &Collider)>,
    dynamic_rigid_body: Query<(Entity, &mut KinematicRigidBody)>,
    mut positions: Query<&mut FixedTransform>,
) {
    for (current, mut rigid_body) in dynamic_rigid_body {
        let (_, global_transform, collider) = transform.get(current).unwrap();
        if collider.disabled {
            continue;
        }

        let collider_transform = global_transform.transform();
        let collider_transform = collider.transform(&collider_transform);

        let mut iter = SubstepIterator::new(
            collider_transform,
            rigid_body.velocity.x * time.delta_time(),
            rigid_body.velocity.y * time.delta_time(),
            rigid_body.velocity.z * time.delta_time(),
        );

        for (other, other_global_transform, other_collider) in transform {
            if current == other || other_collider.disabled {
                continue;
            }

            let other_collider_transform = other_global_transform.transform();
            let other_collider_transform = other_collider.transform(&other_collider_transform);

            let other_rect = Obb::from_transform(
                other_collider_transform.position,
                other_collider_transform.size,
                other_collider_transform.rotation,
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

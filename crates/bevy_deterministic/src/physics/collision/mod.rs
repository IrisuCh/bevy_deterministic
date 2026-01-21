mod aabb;
mod collider;
pub mod event;
mod obb;
mod side;
mod substep;

use bevy::prelude::*;

pub use crate::physics::collision::{aabb::Aabb, collider::Collider, side::CollisionSide};
use crate::{
    math::{FVec3, Fx, fx},
    physics::{
        KinematicRigidBody,
        collision::{obb::Obb, substep::SubstepIterator},
        prelude::event::{CollisionEnter, CollisionExit, CollisionStay},
    },
    resources::FixedTime,
    transform::{FixedGlobalTransform, FixedTransform},
};

pub mod prelude {
    pub use super::*;
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

#[inline]
fn get_rect(global_transform: &FixedGlobalTransform, collider: &Collider) -> Obb {
    let local_transform = global_transform.as_local();
    let collider_transform = collider.transform(&local_transform);
    Obb::from_transform(
        collider_transform.position,
        collider_transform.size,
        collider_transform.rotation,
    )
}

#[inline]
fn get_substep_iterator(
    entity: Entity,
    time: &FixedTime,
    transform: &FixedGlobalTransform,
    rigidbodies: &mut Query<&KinematicRigidBody>,
) -> SubstepIterator {
    let local_transform = transform.as_local();
    if let Ok(rigidbody) = rigidbodies.get_mut(entity) {
        SubstepIterator::new(local_transform, rigidbody.velocity * time.delta_time())
    } else {
        SubstepIterator::with_no_velocity(local_transform)
    }
}

pub(crate) fn apply_physics(
    mut commands: Commands,
    time: Res<FixedTime>,
    mut colliders: Query<(Entity, &FixedGlobalTransform, &mut Collider)>,
    mut rigidbodies: Query<&KinematicRigidBody>,
    mut positions: Query<&mut FixedTransform>,
) {
    let mut iter = colliders.iter_combinations_mut();
    while let Some([(e1, transform1, mut collider1), (e2, transform2, collider2)]) =
        iter.fetch_next()
    {
        if collider1.disabled || collider2.disabled {
            continue;
        }

        let mut iter = get_substep_iterator(e1, &time, transform1, &mut rigidbodies);
        let rect2 = get_rect(transform2, &collider2);
        let Some(collision_info) = iter.next_overlap(&rect2) else {
            if collider1.remove_other(e2) {
                commands.trigger(CollisionExit {
                    entity: e1,
                    other: e2,
                });
            }
            continue;
        };

        if !collider1.trigger && !collider1.fixed {
            let position = &mut positions.get_mut(e1).unwrap();
            position.position -= collision_info.normal * (collision_info.depth - fx!(f32::EPSILON));
        }

        let side = normal_to_side(collision_info.normal);
        if collider1.has_other(e2) {
            commands.trigger(CollisionStay {
                entity: e1,
                side,
                info: collision_info,
            });
        } else {
            collider1.insert_other(e2, side);
            commands.trigger(CollisionEnter {
                entity: e1,
                side,
                info: collision_info,
            });

            commands.trigger(CollisionStay {
                entity: e1,
                side,
                info: collision_info,
            });
        }
    }
}

pub(crate) fn block_rigidbody_movement_along_normal(
    evt: On<CollisionStay>,
    mut rigidbodies: Query<(&mut KinematicRigidBody, &Collider)>,
) {
    if let Ok((mut rigidbody, collider)) = rigidbodies.get_mut(evt.entity)
        && !collider.trigger
    {
        let normal_vel = rigidbody.velocity.dot(evt.info.normal);
        if normal_vel > Fx::ZERO {
            rigidbody.velocity -= evt.info.normal * normal_vel;
        }
    }
}

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
    transform::{FVec3, FixedGlobalTransform, FixedTransform},
};

pub mod prelude {
    pub use super::*;
}

#[derive(Component, Reflect, Debug, Default)]
#[require(FixedTransform)]
pub struct Collider;

fn normal_to_side(normal: FVec3) -> CollisionSide {
    let abs_normal = FVec3::new_fixed(normal.x.abs(), normal.y.abs(), normal.z.abs());

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
    time: Res<Time<Fixed>>,
    transform: Query<(Entity, &FixedGlobalTransform), With<Collider>>,
    dynamic_rigid_body: Query<(Entity, &mut KinematicRigidBody)>,
    mut positions: Query<&mut FixedTransform>,
) {
    let delta = Fx::from_num(time.delta_secs());
    for (current, mut rigid_body) in dynamic_rigid_body {
        let (_, global_transform) = transform.get(current).unwrap();
        let mut iter = SubstepIterator::new(
            global_transform.transform(),
            rigid_body.velocity.x * delta,
            rigid_body.velocity.y * delta,
            rigid_body.velocity.z * delta,
        );

        let rect = Obb::from_transform(
            global_transform.position(),
            global_transform.size(),
            global_transform.rotation(),
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

            println!("Collision detected between {rect:#?} and {other_rect:#?}");

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
    // normal указывает ОТ динамического К статическому
    let vel_toward_static = velocity.dot(normal);

    // Если движемся К статическому (normal_vel > 0) — блокируем
    if vel_toward_static > Fx::ZERO {
        *velocity -= normal * vel_toward_static;
    }
    // Если normal_vel < 0 — движемся ОТ статического, разрешаем
}

//fn block_movement_along_normal(normal: FVec3, velocity: &mut FVec3) {
//    // Проекция скорости на нормаль
//    let normal_vel = velocity.dot(normal);
//
//    // Если движемся В нормаль (внутрь объекта) - обнуляем эту компоненту
//    if normal_vel < Fx::ZERO {
//        *velocity -= normal * normal_vel;
//    }
//}

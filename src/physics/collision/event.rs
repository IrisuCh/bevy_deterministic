use bevy::prelude::*;
use fixed::types::I32F32;

use crate::{
    physics::{
        collision::{CollisionSide, aabb::Aabb},
        rigidbody::KinematicRigidBody,
    },
    transform::FixedTransform,
};

#[derive(EntityEvent)]
pub struct CollisionEnter {
    pub entity: Entity,
    pub side: CollisionSide,
    pub offset: I32F32,
}

#[derive(EntityEvent)]
pub struct CollisionStay {
    pub entity: Entity,
    pub side: CollisionSide,
    pub offset: I32F32,
}

#[derive(EntityEvent)]
pub struct CollisionExit {
    pub entity: Entity,
    pub other: Entity,
}

pub(super) fn trigger_enter(
    entity: Entity,
    side: CollisionSide,
    offset: I32F32,
    commands: &mut Commands,
    transform: &mut FixedTransform,
    rect: &Aabb,
    other_rect: &Aabb,
) {
    match side {
        CollisionSide::Left => transform.position.x = other_rect.max.x,
        CollisionSide::Right => transform.position.x = other_rect.min.x - rect.w(),

        CollisionSide::Bottom => transform.position.y = other_rect.max.y,
        CollisionSide::Top => transform.position.y = other_rect.min.y - rect.h(),

        CollisionSide::Back => transform.position.z = other_rect.max.z,
        CollisionSide::Front => transform.position.z = other_rect.min.z - rect.d(),
    }

    commands.trigger(CollisionEnter {
        entity,
        side,
        offset,
    });
}

pub(super) fn trigger_stay(
    entity: Entity,
    side: CollisionSide,
    offset: I32F32,
    commands: &mut Commands,
    rigid_body: &mut KinematicRigidBody,
    transform: &mut FixedTransform,
) {
    match side {
        CollisionSide::Left => rigid_body.velocity.clamp_positive_x(),
        CollisionSide::Right => rigid_body.velocity.clamp_negative_x(),

        CollisionSide::Top => rigid_body.velocity.clamp_negative_y(),
        CollisionSide::Bottom => rigid_body.velocity.clamp_positive_y(),

        CollisionSide::Front => rigid_body.velocity.clamp_negative_z(),
        CollisionSide::Back => rigid_body.velocity.clamp_positive_z(),
    }

    if !rigid_body.is_offset_applied(side) {
        match side {
            CollisionSide::Left => transform.position.x += offset,
            CollisionSide::Right => transform.position.x -= offset,

            CollisionSide::Bottom => transform.position.y += offset,
            CollisionSide::Top => transform.position.y -= offset,

            CollisionSide::Back => transform.position.z -= offset,
            CollisionSide::Front => transform.position.z += offset,
        }
    }

    commands.trigger(CollisionStay {
        entity,
        side,
        offset,
    });
}

pub(super) fn trigger_exit(entity: Entity, other: Entity, commands: &mut Commands) {
    commands.trigger(CollisionExit { entity, other });
}

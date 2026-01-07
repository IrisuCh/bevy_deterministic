use bevy::prelude::*;

use crate::physics::collision::CollisionSide;

#[derive(EntityEvent)]
pub struct CollisionEnter {
    pub entity: Entity,
    pub side: CollisionSide,
}

#[derive(EntityEvent)]
pub struct CollisionStay {
    pub entity: Entity,
    pub side: CollisionSide,
}

#[derive(EntityEvent)]
pub struct CollisionExit {
    pub entity: Entity,
    pub other: Entity,
}

pub(super) fn trigger_enter(entity: Entity, side: CollisionSide, commands: &mut Commands) {
    commands.trigger(CollisionEnter { entity, side });
}

pub(super) fn trigger_stay(entity: Entity, side: CollisionSide, commands: &mut Commands) {
    commands.trigger(CollisionStay { entity, side });
}

pub(super) fn trigger_exit(entity: Entity, other: Entity, commands: &mut Commands) {
    commands.trigger(CollisionExit { entity, other });
}

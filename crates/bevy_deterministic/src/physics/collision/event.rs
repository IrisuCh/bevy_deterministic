use bevy::prelude::*;

use crate::physics::collision::{CollisionSide, obb::CollisionInfo};

#[derive(EntityEvent)]
pub struct CollisionEnter {
    pub entity: Entity,
    pub side: CollisionSide,
    pub info: CollisionInfo,
}

#[derive(EntityEvent)]
pub struct CollisionStay {
    pub entity: Entity,
    pub side: CollisionSide,
    pub info: CollisionInfo,
}

#[derive(EntityEvent)]
pub struct CollisionExit {
    pub entity: Entity,
    pub other: Entity,
}

use bevy::prelude::*;

use crate::{math::FVec3, physics::collision::Collider};

#[derive(Component, Reflect, Debug, Default)]
#[require(Collider)]
pub struct KinematicRigidBody {
    pub velocity: FVec3,
    pub freeze: bool,
}

impl KinematicRigidBody {
    #[must_use]
    pub fn freezed() -> Self {
        Self {
            velocity: FVec3::ZERO,
            freeze: true,
        }
    }
}

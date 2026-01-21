use bevy::prelude::*;
use fx::IntoFx;

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

    pub fn set_velocity_xz3(&mut self, vec: FVec3) {
        self.velocity.x = vec.x;
        self.velocity.z = vec.z;
    }

    pub fn set_velocity_xz(&mut self, x: impl IntoFx, z: impl IntoFx) {
        self.velocity.x = x.into_fx();
        self.velocity.z = z.into_fx();
    }
}

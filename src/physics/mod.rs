pub mod collision;
mod debug;
mod rigidbody;
mod velocity;

use bevy::prelude::*;
pub use debug::PhysicsDebugManager;
pub(crate) use debug::draw_collider_debug_lines;
pub use rigidbody::KinematicRigidBody;
pub use velocity::Velocity;

use crate::transform::Position;
pub mod prelude {
    pub use super::{KinematicRigidBody, PhysicsDebugManager, Velocity, collision::prelude::*};
}

pub(crate) fn apply_velocity(mut entities: Query<(&mut Position, &KinematicRigidBody)>) {
    for (mut position, rigid_body) in &mut entities {
        position.x += rigid_body.velocity.x;
        position.y += rigid_body.velocity.y;
        position.z += rigid_body.velocity.z;
    }
}

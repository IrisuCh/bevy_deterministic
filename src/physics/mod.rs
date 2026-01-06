pub mod collision;
mod debug;
mod rigidbody;
mod velocity;

use bevy::{log::tracing_subscriber::field::delimited::Delimited, prelude::*};
pub use debug::PhysicsDebugManager;
pub(crate) use debug::draw_collider_debug_lines;
pub use rigidbody::KinematicRigidBody;
pub use velocity::Velocity;

use crate::{fx, transform::Position};
pub mod prelude {
    pub use super::{KinematicRigidBody, PhysicsDebugManager, Velocity, collision::prelude::*};
}

pub(crate) fn apply_velocity(
    time: Res<Time<Fixed>>,
    mut entities: Query<(&mut Position, &KinematicRigidBody)>,
) {
    let delta = fx::from_num(time.delta_secs());
    for (mut position, rigid_body) in &mut entities {
        position.x += rigid_body.velocity.x * delta;
        position.y += rigid_body.velocity.y * delta;
        position.z += rigid_body.velocity.z * delta;
    }
}

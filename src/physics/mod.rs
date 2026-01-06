pub mod collision;
mod debug;
mod rigidbody;
mod velocity;

use bevy::prelude::*;
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
    mut entities: Query<(&mut Position, &mut KinematicRigidBody)>,
) {
    for (mut position, mut rigid_body) in &mut entities {
        position.x += rigid_body.velocity.x;
        position.y += rigid_body.velocity.y;
        position.z += rigid_body.velocity.z;
        rigid_body.velocity.y -= fx::from_num(10.0 * time.delta_secs());
    }
}

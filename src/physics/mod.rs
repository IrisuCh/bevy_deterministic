pub mod collision;
mod debug;
mod rigidbody;
mod velocity;

use bevy::prelude::*;
pub use debug::PhysicsDebugManager;
pub(crate) use debug::draw_collider_debug_lines;
pub use rigidbody::KinematicRigidBody;
pub use velocity::Velocity;

use crate::{Fx, transform::FixedTransform};
pub mod prelude {
    pub use super::{KinematicRigidBody, PhysicsDebugManager, Velocity, collision::prelude::*};
}

pub(crate) fn apply_velocity(
    time: Res<Time<Fixed>>,
    mut entities: Query<(&mut FixedTransform, &KinematicRigidBody)>,
) {
    let delta = Fx::from_num(time.delta_secs());
    for (mut transform, rigid_body) in &mut entities {
        transform.position.x += rigid_body.velocity.x * delta;
        transform.position.y += rigid_body.velocity.y * delta;
        transform.position.z += rigid_body.velocity.z * delta;
    }
}

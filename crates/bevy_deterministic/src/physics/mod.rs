pub mod collision;
mod debug;
mod rigidbody;

use bevy::prelude::*;
pub use debug::PhysicsDebugManager;
pub(crate) use debug::draw_collider_debug_lines;
pub use rigidbody::KinematicRigidBody;

use crate::{resources::FixedTime, transform::FixedTransform};
pub mod prelude {
    pub use super::{KinematicRigidBody, PhysicsDebugManager, collision::prelude::*};
}

pub(crate) fn apply_velocity(
    time: Res<FixedTime>,
    mut entities: Query<(&mut FixedTransform, &KinematicRigidBody)>,
) {
    for (mut transform, rigid_body) in &mut entities {
        transform.position.x += rigid_body.velocity.x * time.delta_time();
        transform.position.y += rigid_body.velocity.y * time.delta_time();
        transform.position.z += rigid_body.velocity.z * time.delta_time();
    }
}

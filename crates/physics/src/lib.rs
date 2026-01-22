pub mod collision;
mod rigidbody;

use bridge::BridgePlugin;
pub use rigidbody::Rigidbody;
pub(crate) use rigidbody::{apply_material_friction, apply_velocity};

pub mod prelude {
    pub use super::{Rigidbody, collision::prelude::*};
}

use bevy_deterministic::main::{DPlugin, DeterministicWorld, input::UserInput, schedule::Physics};

use crate::prelude::{apply_physics, block_rigidbody_movement_along_normal};

pub struct PhysicsPlugin;
impl<I: UserInput> DPlugin<I> for PhysicsPlugin {
    fn build(&self, world: &mut DeterministicWorld<I>) {
        world.add_systems(
            Physics,
            (apply_physics, apply_material_friction, apply_velocity),
        );

        world.add_observer(block_rigidbody_movement_along_normal);

        world.add_plugin(BridgePlugin::<I>::default());
    }
}

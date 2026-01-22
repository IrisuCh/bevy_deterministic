#![allow(clippy::needless_pass_by_value)]

mod map;
mod time;
pub mod transform;

use bevy_deterministic::main::{DPlugin, DeterministicWorld, input::UserInput, schedule::Physics};
pub use map::Map;
pub use time::{Time, sync_time};

use crate::transform::{sync_fixed_global_transforms, sync_fixed_transforms};

pub struct BridgePlugin<I: UserInput> {
    _marker: std::marker::PhantomData<I>,
}

impl<I: UserInput> Default for BridgePlugin<I> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<I: UserInput> DPlugin<I> for BridgePlugin<I> {
    fn build(&self, world: &mut DeterministicWorld<I>) {
        world.init_resource::<Time>();
        world.add_systems(
            Physics,
            (sync_fixed_global_transforms, sync_fixed_transforms),
        );
    }
}

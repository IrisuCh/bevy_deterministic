#![no_std]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]
#![deny(clippy::float_arithmetic)]
#![deny(clippy::float_cmp)]
#![deny(clippy::cast_possible_truncation)]
#![deny(clippy::cast_precision_loss)]
//#![deny(clippy::as_conversions)]
#![deny(clippy::disallowed_types)]

pub mod input;
pub mod main;
pub mod map;

use bevy::prelude::*;
pub mod math {
    pub use fixed_math::*;
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerLogicSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InternalDeterministicSet;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, InternalDeterministicSet.after(PlayerLogicSet));

        //app.add_systems(
        //    FixedUpdate,
        //    ((
        //        apply_physics,
        //        apply_material_friction,
        //        apply_velocity,
        //        sync_fixed_global_transforms,
        //        sync_fixed_transforms,
        //    )
        //        .chain()
        //        .in_set(InternalDeterministicSet),)
        //        .chain(),
        //);

        //app.add_observer(block_rigidbody_movement_along_normal);
    }
}

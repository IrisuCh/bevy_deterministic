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

pub mod input;
pub mod physics;
mod sync;
pub mod tilemap;
pub mod transform;

use bevy::prelude::*;
pub use sync::SyncTarget;

use crate::{
    physics::{
        PhysicsDebugManager, apply_velocity, draw_collider_debug_lines, prelude::apply_physics,
    },
    tilemap::{CollisionBackend, on_chunk_spawn, set_tiles_position, split_by_chunks},
    transform::{sync_fixed_global_transforms, sync_fixed_transforms, sync_transform},
};

pub type Fx = ::fixed::types::I32F32;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerLogicSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InternalDeterministicSet;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsDebugManager>();

        app.configure_sets(FixedUpdate, InternalDeterministicSet.after(PlayerLogicSet));

        app.add_systems(
            FixedUpdate,
            (
                (
                    set_tiles_position,
                    split_by_chunks,
                    on_chunk_spawn::<CollisionBackend>,
                )
                    .chain()
                    .in_set(PlayerLogicSet),
                (
                    apply_physics,
                    apply_velocity,
                    sync_fixed_global_transforms,
                    sync_fixed_transforms,
                )
                    .chain()
                    .in_set(InternalDeterministicSet),
            )
                .chain(),
        );

        app.add_systems(Update, (sync_transform, draw_collider_debug_lines).chain());
    }
}

/*
 * Some work
 * Edit velocity
 * Apply physics
 * Apply velocity
 * Sync
 */

#[macro_export]
macro_rules! fx {
    ($n:expr) => {
        $crate::Fx::from_num($n)
    };
}

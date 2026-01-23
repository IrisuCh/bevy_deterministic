#![allow(clippy::needless_pass_by_value)]
#![no_std]

pub mod collision;
mod rigidbody;

use bevy::{
    app::{Plugin, Update},
    color::palettes::css::GREEN,
    ecs::{
        resource::Resource,
        system::{Query, Res},
    },
    gizmos::gizmos::Gizmos,
    transform::components::Transform,
};
use fixed_math::{FQuat, FVec3, const_fx};
pub use rigidbody::Rigidbody;
pub(crate) use rigidbody::{apply_material_friction, apply_velocity};

pub mod prelude {
    pub use super::{Rigidbody, collision::prelude::*};
}

use whitelace_core::main::schedule::Physics;
use whitelace_sync::{MultiworldApp, SyncTarget, WorldLabel, Worlds};
use whitelace_time::TimePlugin;
use whitelace_transform::{FixedGlobalTransform, TransformPlugin};

use crate::prelude::{Collider, apply_physics, block_rigidbody_movement_along_normal};

pub struct PhysicsPlugin<W: WorldLabel + Default> {
    _phantom: core::marker::PhantomData<W>,
}

impl<W: WorldLabel + Default> Default for PhysicsPlugin<W> {
    fn default() -> Self {
        Self {
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<W: WorldLabel + Default> Plugin for PhysicsPlugin<W> {
    fn build(&self, app: &mut bevy::app::App) {
        let is_transform_added = app.is_plugin_added::<TransformPlugin<W>>();
        let is_time_added = app.is_plugin_added::<TimePlugin<W>>();

        if !is_time_added {
            app.add_plugins(TimePlugin::<W>::default());
        }

        if !is_transform_added {
            app.add_plugins(TransformPlugin::<W>::default());
        }

        app.add_world_systems(
            W::default(),
            Physics,
            (apply_physics, apply_material_friction, apply_velocity),
        );

        app.add_world_observer(W::default(), block_rigidbody_movement_along_normal);

        app.init_resource::<PhysicsDebugManager>();
        app.add_systems(Update, draw_collider_debug_lines::<W>);
    }
}

#[derive(Resource, Default)]
pub struct PhysicsDebugManager {
    pub draw_collider_lines: bool,
}

fn draw_collider_debug_lines<W: WorldLabel + Default>(
    manager: Res<PhysicsDebugManager>,
    mut gizmos: Gizmos,
    worlds: Res<Worlds>,
    query: Query<&SyncTarget>,
) {
    if !manager.draw_collider_lines {
        return;
    }

    let world = worlds.get(W::default()).unwrap();
    for sync_target in query {
        let entity = world.get_entity(sync_target.0).unwrap();
        let Some(collider) = entity.get::<Collider>() else {
            continue;
        };

        let fixed_transform = entity.get::<FixedGlobalTransform>().unwrap();
        let (pos, rotation, size): (FVec3, FQuat, FVec3) =
            collider.transform(&fixed_transform.as_local()).into();
        let position = (pos + size / const_fx!(2)).as_vec3();

        gizmos.cube(
            Transform::from_translation(position)
                .with_scale(size.as_vec3())
                .with_rotation(rotation.as_quat()),
            GREEN,
        );
    }
}

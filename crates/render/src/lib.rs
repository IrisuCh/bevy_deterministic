#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;

use crate::physics_debug::{PhysicsDebugManager, draw_collider_debug_lines};

mod physics_debug;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsDebugManager>();
        app.add_systems(Update, draw_collider_debug_lines);
    }
}

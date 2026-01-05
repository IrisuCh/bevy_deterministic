#![allow(clippy::cast_precision_loss)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::similar_names)]

pub mod collision;
mod determinism;
mod map;
mod physics;
mod simulation_input;
mod simulation_sync;

use crate::determinism::plugin::GameplayPlugin;
use crate::simulation_input::SimulationInputPlugin;
use crate::simulation_sync::SimulationSyncPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins(GameplayPlugin);
    app.add_plugins(SimulationInputPlugin);
    app.add_plugins(SimulationSyncPlugin);

    app.add_plugins(EguiPlugin::default());
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, start);

    app.run();
}

fn start(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 100.0, 1000.0),
        //Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

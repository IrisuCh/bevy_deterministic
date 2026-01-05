mod determinism;
mod simulation_input;
mod simulation_sync;
mod map;
pub mod collision;
mod physics;

use crate::determinism::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::determinism::plugin::GameplayPlugin;
use crate::simulation_input::SimulationInputPlugin;
use crate::simulation_sync::SimulationSyncPlugin;

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

fn start(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
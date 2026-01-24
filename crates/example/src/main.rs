use bevy::{DefaultPlugins, app::App};
use whitelace_plugin::WhitelacePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(WhitelacePlugin);
    app.run();
}

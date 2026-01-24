#![allow(clippy::needless_pass_by_value)]

use bevy::app::{App, Plugin};
use whitelace_physics::PhysicsPlugin;
use whitelace_sync::{MultiworldApp, SyncPlugin, WorldLabel};
use whitelace_time::TimePlugin;
use whitelace_transform::TransformPlugin;

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LogicWorld;
impl WorldLabel for LogicWorld {
    #[doc = r" Clones this `"]
    #[doc = stringify!(WorldLabel)]
    #[doc = r"`."]
    fn dyn_clone(&self) -> Box<dyn WorldLabel> {
        Box::new(LogicWorld)
    }
}

pub struct WhitelacePlugin;
impl Plugin for WhitelacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SyncPlugin);
        app.add_world(LogicWorld);

        app.add_plugins(TimePlugin::<LogicWorld>::default());
        app.add_plugins(TransformPlugin::<LogicWorld>::default());
        app.add_plugins(PhysicsPlugin::<LogicWorld>::default());
    }
}

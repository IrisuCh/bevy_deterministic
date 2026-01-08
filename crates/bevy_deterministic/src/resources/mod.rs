mod time;

use bevy::prelude::*;
pub use time::FixedTime;

use crate::resources::time::sync_time;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FixedTime>();

        app.add_systems(PreUpdate, sync_time);
    }
}

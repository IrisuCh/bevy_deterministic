#![allow(clippy::needless_pass_by_value)]

use bevy::prelude::*;
use bridge::transform::FixedTransform;
use sync::{MultiworldApp, SyncPlugin, SyncTarget, WorldLabel, WorldQuery};

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
        app.add_sync_system(sync_transform);
    }
}

fn sync_transform(
    from: WorldQuery<&FixedTransform, (), LogicWorld>,
    mut to: WorldQuery<(&mut Transform, &SyncTarget)>,
) {
    for (mut transform, sync) in &mut to.iter_mut() {
        let fixed_transform = from.get(sync.0).unwrap();

        transform.translation =
            fixed_transform.position.as_vec3() + fixed_transform.size.as_vec3() / 2.0;
        transform.scale = fixed_transform.size.as_vec3();
        transform.rotation = fixed_transform.rotation.as_quat();
    }
}

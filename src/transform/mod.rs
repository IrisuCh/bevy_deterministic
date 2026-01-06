mod position;
mod size;

use bevy::prelude::*;

use crate::sync::SyncTarget;
pub use crate::transform::{
    position::{GlobalPosition, Position},
    size::Size,
};

pub mod prelude {
    pub use super::*;
}

pub(crate) fn sync_global_positions(
    query: Query<(&mut GlobalPosition, &Position), Changed<Position>>,
) {
    for (mut global, local) in query {
        global.x = local.x;
        global.y = local.y;
        global.z = local.z;
    }
}

pub(crate) fn sync_positions(
    query: Query<(Entity, &Children), Changed<Position>>,
    mut positions: Query<(&mut GlobalPosition, &Position)>,
) {
    for (parent, children) in query {
        let (global_x, global_y, global_z) = {
            let global = positions.get(parent).unwrap().0;
            (global.x, global.y, global.z)
        };
        for entity in children {
            if let Ok((mut child_global, child_local)) = positions.get_mut(*entity) {
                child_global.x = global_x + child_local.x;
                child_global.y = global_y + child_local.y;
                child_global.z = global_z + child_local.z;
            }
        }
    }
}

pub(crate) fn sync_transform(
    from: Query<(&GlobalPosition, &Size)>,
    mut to: Query<(&mut Transform, &SyncTarget)>,
) {
    for (mut transform, sync) in &mut to {
        let (position, size) = from.get(sync.0).unwrap();

        // Сдвигаем позицию к центру
        transform.translation = position.as_vec3() + size.as_vec3() / 2.0;
        transform.scale = size.as_vec3();
    }
}

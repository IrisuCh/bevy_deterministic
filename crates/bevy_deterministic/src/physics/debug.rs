use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{
    math::{FQuat, FVec3, fx},
    physics::prelude::Collider,
    transform::FixedGlobalTransform,
};

#[derive(Resource, Default)]
pub struct PhysicsDebugManager {
    pub draw_collider_lines: bool,
}

pub(crate) fn draw_collider_debug_lines(
    manager: Res<PhysicsDebugManager>,
    mut gizmos: Gizmos,
    query: Query<(&FixedGlobalTransform, &Collider)>,
) {
    if !manager.draw_collider_lines {
        return;
    }

    for (transform, collider) in &query {
        let (pos, rotation, size): (FVec3, FQuat, FVec3) =
            collider.transform(&transform.as_local()).into();
        let position = (pos + size / fx!(2.0)).as_vec3();

        gizmos.cube(
            Transform::from_translation(position)
                .with_scale(size.as_vec3())
                .with_rotation(rotation.as_quat()),
            GREEN,
        );
    }
}

use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{fx, physics::prelude::Collider, transform::FixedTransform};

#[derive(Resource, Default)]
pub struct PhysicsDebugManager {
    pub draw_collider_lines: bool,
}

pub(crate) fn draw_collider_debug_lines(
    manager: Res<PhysicsDebugManager>,
    mut gizmos: Gizmos,
    query: Query<&FixedTransform, With<Collider>>,
) {
    if manager.draw_collider_lines {
        return;
    }

    for transform in &query {
        let pos = transform.position;
        let size = transform.size;
        let rotation = transform.rotation;

        let position = (pos + size / fx!(2.0)).as_vec3();

        gizmos.cuboid(
            Transform::from_translation(position)
                .with_scale(size.as_vec3())
                .with_rotation(rotation.as_quat()),
            GREEN,
        );
    }
}

use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{physics::prelude::Collider, transform::FixedTransform};

#[derive(Resource, Default)]
pub struct PhysicsDebugManager {
    pub draw_collider_lines: bool,
}

pub(crate) fn draw_collider_debug_lines(
    manager: Res<PhysicsDebugManager>,
    mut gizmos: Gizmos,
    query: Query<&FixedTransform, With<Collider>>,
) {
    if !manager.draw_collider_lines {
        return;
    }

    for transform in &query {
        let pos = transform.position;
        let size = transform.size;
        let rotation = transform.rotation;

        let x = pos.x_f32() + size.x_f32() / 2.0;
        let y = pos.y_f32() + size.y_f32() / 2.0;
        let z = pos.z_f32() + size.z_f32() / 2.0;

        gizmos.cuboid(
            Transform::from_xyz(x, y, z)
                .with_scale(size.as_vec3())
                .with_rotation(rotation.as_quat()),
            GREEN,
        );
    }
}

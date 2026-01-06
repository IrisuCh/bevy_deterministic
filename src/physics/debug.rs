use bevy::{color::palettes::css::GREEN, prelude::*};

use crate::{
    physics::prelude::Collider,
    transform::{GlobalPosition, Size},
};

#[derive(Resource, Default)]
pub struct PhysicsDebugManager {
    pub draw_collider_lines: bool,
}

pub(crate) fn draw_collider_debug_lines(
    mut gizmos: Gizmos,
    query: Query<(&GlobalPosition, &Size), With<Collider>>,
) {
    for (pos, size) in &query {
        let x = pos.x_f32() + size.x_f32() / 2.0;
        let y = pos.y_f32() + size.y_f32() / 2.0;
        let z = pos.z_f32() + size.z_f32() / 2.0;
        let w = size.x_f32();
        let h = size.y_f32();
        let d = size.z_f32();

        gizmos.cuboid(
            Transform::from_xyz(x, y, z).with_scale(Vec3::new(w, h, d)),
            GREEN,
        );
    }
}

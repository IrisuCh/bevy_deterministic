use crate::collision::Collider;
use crate::determinism::plugin::{PlayerMarker, TestMarker};
use crate::determinism::transform::{GlobalPosition, Size};
use crate::map::TileIndex;
use bevy::color::palettes::basic::{GREEN, PURPLE, RED};
use bevy::color::palettes::css::VIOLET;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerVisualMarker;

#[derive(Component, Deref, DerefMut)]
pub struct SyncTarget(Entity);

pub struct SimulationSyncPlugin;

impl Plugin for SimulationSyncPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sync_player_spawn, sync_test_spawn, sync_tile_spawn).chain(),
        );
        app.add_systems(Update, draw_collider_debug_lines);
        app.add_systems(PostUpdate, sync_transform);
    }
}

fn sync_transform(
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

fn sync_player_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_players: Query<(Entity, &GlobalPosition, &Size), Added<PlayerMarker>>,
) {
    for (entity, position, size) in added_players {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            //Mesh2d(meshes.add(Rectangle::default())),
            //MeshMaterial2d(materials.add(Color::from(VIOLET))),
            MeshMaterial3d(materials.add(Color::from(VIOLET))),
            Transform::from_translation(position.as_vec3()).with_scale(size.as_vec3()),
            SyncTarget(entity),
            PlayerVisualMarker,
        ));
    }
}

fn sync_test_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tests: Query<(Entity, &GlobalPosition, &Size), Added<TestMarker>>,
) {
    for (entity, position, size) in tests {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(PURPLE))),
            Transform::from_translation(position.as_vec3()).with_scale(size.as_vec3()),
            SyncTarget(entity),
        ));
    }
}

fn sync_tile_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tiles: Query<(Entity, &GlobalPosition, &Size), Added<TileIndex>>,
) {
    for (entity, position, size) in tiles {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::from(RED))),
            Transform::from_translation(position.as_vec3()).with_scale(size.as_vec3()),
            SyncTarget(entity),
        ));
    }
}

pub fn draw_collider_debug_lines(
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

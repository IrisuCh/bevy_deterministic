use crate::collision::{Collider, apply_physics};
use crate::determinism::transform::{Position, Size, sync_global_positions, sync_positions};
use crate::map::{CollisionBackend, on_chunk_spawn, set_tiles_position, split_by_chunks};
use crate::physics::KinematicRigidBody;
use crate::simulation_input::{Action, InputSnapshot, WorldMousePosition};
use bevy::prelude::*;
use fixed::types::I32F32;

#[derive(Component, Reflect, Default)]
pub struct TestMarker;

#[derive(Component, Reflect, Default)]
#[require(EntityMarker)]
pub struct PlayerMarker;

#[derive(Component, Reflect, Default)]
#[require(KinematicRigidBody)]
pub struct EntityMarker;

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMousePosition>();
        app.init_resource::<InputSnapshot>();
        app.add_systems(Startup, start);
        app.add_systems(
            FixedUpdate,
            (
                set_tiles_position,
                split_by_chunks,
                on_chunk_spawn::<CollisionBackend>,
                apply_player_movement,
                apply_physics,
                update_entity_movement,
                sync_global_positions,
                sync_positions,
            )
                .chain(),
        );
    }
}

fn start(mut commands: Commands) {
    for x in 0..10 {
        let x = -300 + 64 * x;
        commands.spawn((
            Collider::default(),
            Size::new_f32(64.0, 64.0, 64.0),
            Position::new_f32(x as f32, 0.0, 0.0),
        ));
    }

    commands.spawn((
        Collider::default(),
        Size::new_f32(64.0, 64.0, 64.0),
        Position::new_f32(-64.0, 64.0, 0.0),
    ));

    commands
        .spawn((
            PlayerMarker,
            Collider::default(),
            Size::new_f32(128.0, 128.0, 64.0),
            Position::new_f32(0.0, 1000.0, 0.0),
            Name::new("Player-Gameplay"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Position::new_f32(0.0, 0.0, 128.0),
                Size::new_f32(64.0, 64.0, 64.0),
                TestMarker,
                Name::new("Test"),
            ));
        });

    /*
    let tilemap_entity = commands.spawn_empty().id();
    let tile_size = TileSize::new(10.0, 10.0);
    let map_size = TilemapSize::new(10, 10);
    let mut storage = TilemapStorage::new(&map_size);

    for x in 3..8 {
        for y in 3..8 {
            storage.create_tile(10, TilePosition::new(x, y),  TileIndex(0), tilemap_entity, &mut commands);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        storage,
        map_size,
        tile_size,
        tile_padding: TilePadding {
            x: I32F32::from_num(5.0),
            y: I32F32::from_num(5.0),
        },
        position: Position::default(),
    });
     */
}

fn apply_player_movement(
    input: Res<InputSnapshot>,
    mouse_pos: Res<WorldMousePosition>,
    player: Single<(&mut Position, &mut KinematicRigidBody), With<PlayerMarker>>,
) {
    let (mut position, mut rigid_body) = player.into_inner();
    println!("Player position: {position:?}");
    for action in input.inner() {
        match action {
            Action::Click => {
                position.x = mouse_pos.x();
                position.y = mouse_pos.y();
                position.z = I32F32::ZERO;
                rigid_body.velocity.y = I32F32::ZERO;
            }
            Action::Jump => rigid_body.velocity.y = I32F32::from_num(6.0),
            Action::Fire => println!("Fire"),
            Action::Left => rigid_body.velocity.x = I32F32::const_from_int(2),
            Action::Right => rigid_body.velocity.x = I32F32::const_from_int(-2),
            Action::Idle => rigid_body.velocity.x = I32F32::ZERO,
        }
    }
}

fn update_entity_movement(
    time: Res<Time<Fixed>>,
    mut entities: Query<(&mut Position, &mut KinematicRigidBody)>,
) {
    for (mut position, mut rigid_body) in &mut entities {
        position.x += rigid_body.velocity.x;
        position.y += rigid_body.velocity.y;
        position.z += rigid_body.velocity.z;
        rigid_body.velocity.y -= I32F32::from_num(10.0 * time.delta_secs());
    }
}

#![allow(clippy::type_complexity)]
#![allow(clippy::missing_panics_doc)]
#![no_std]

mod chunk;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use whitelace_core::{main::schedule::FixedUpdate, math::Fx};
use whitelace_physics::collision::Collider;
use whitelace_sync::{MultiworldApp, WorldLabel};
use whitelace_transform::FixedTransform;

use crate::chunk::split_by_chunks;
pub use crate::chunk::{Chunk, ChunkStorage};

#[derive(Component, Reflect)]
pub struct TilemapSize {
    width: u32,
    height: u32,
}

impl TilemapSize {
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct TileSize {
    x: Fx,
    y: Fx,
}

impl TileSize {
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: Fx::from_num(x),
            y: Fx::from_num(y),
        }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct TilePadding {
    pub x: Fx,
    pub y: Fx,
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[require(FixedTransform)]
pub struct TileIndex(pub u32);

#[derive(Component, Reflect)]
pub struct TilePosition {
    x: u32,
    y: u32,
}

impl TilePosition {
    #[must_use]
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Bundle)]
pub struct TilemapBundle {
    pub storage: TilemapStorage,
    pub map_size: TilemapSize,
    pub tile_size: TileSize,
    pub tile_padding: TilePadding,
    //pub position: FVec3,
}

#[derive(Component, Reflect)]
#[require(ChunkStorage)]
pub struct TilemapStorage(Vec<Option<Entity>>);

impl TilemapStorage {
    #[must_use]
    pub fn new(map_size: &TilemapSize) -> Self {
        let capacity = map_size.width as usize * map_size.height as usize;
        Self(vec![None; capacity])
    }

    pub fn create_tile(
        &mut self,
        width: usize,
        position: TilePosition,
        index: TileIndex,
        map: Entity,
        commands: &mut Commands,
    ) {
        let pos = position.y as usize * width + position.x as usize;
        assert!(
            self.0[pos].is_none(),
            "Tile at {}, {} already exist",
            position.x,
            position.y
        );

        let tile = commands
            .spawn((
                position,
                index,
                ChildOf(map),
                //Position::default(),
                //Size::default(),
                Name::new("Tile"),
            ))
            .id();

        self.0[pos] = Some(tile);
    }
}

fn set_tiles_position(
    storages: Query<(&TileSize, &TilePadding), With<TilemapStorage>>,
    tiles: Query<
        (&mut FixedTransform, &TilePosition, &ChildOf),
        Or<(Changed<FixedTransform>, Changed<TilePosition>)>,
    >,
) {
    for (mut transform, tile_pos, parent) in tiles {
        let (tile_size, tile_padding) = storages.get(parent.0).unwrap();
        transform.size.x = tile_size.x;
        transform.size.y = tile_size.y;
        transform.position.x = Fx::from_num(tile_pos.x) * (tile_size.x + tile_padding.x);
        transform.position.y = Fx::from_num(tile_pos.y) * (tile_size.y + tile_padding.y);
    }
}

pub trait OnChunkSpawn {
    fn on_chunk_spawn(commands: &mut Commands, entity: Entity);
}

pub(crate) fn on_chunk_spawn<T: OnChunkSpawn>(
    mut commands: Commands,
    query: Query<Entity, Added<Chunk>>,
) {
    for entity in query {
        T::on_chunk_spawn(&mut commands, entity);
    }
}

pub struct CollisionBackend;
impl OnChunkSpawn for CollisionBackend {
    fn on_chunk_spawn(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).insert(Collider::default());
    }
}

pub struct TilemapPlugin<W: WorldLabel> {
    _phantom: core::marker::PhantomData<W>,
}

impl<W: WorldLabel + Default> Plugin for TilemapPlugin<W> {
    fn build(&self, app: &mut App) {
        app.add_world_systems(
            W::default(),
            FixedUpdate,
            (
                set_tiles_position,
                split_by_chunks,
                on_chunk_spawn::<CollisionBackend>,
            ),
        );
    }
}

mod chunk;

use bevy::prelude::*;
use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

pub(crate) use crate::tilemap::chunk::split_by_chunks;
pub use crate::tilemap::chunk::{Chunk, ChunkStorage};
use crate::{
    physics::collision::Collider,
    transform::{Position, Size},
};

#[derive(Component, Reflect)]
pub struct TilemapSize {
    width: u32,
    height: u32,
}

impl TilemapSize {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct TileSize {
    x: I32F32,
    y: I32F32,
}

impl TileSize {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: I32F32::from_num(x),
            y: I32F32::from_num(y),
        }
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Default, Debug, Clone)]
#[reflect(opaque)]
#[reflect(Serialize, Deserialize)]
pub struct TilePadding {
    pub x: I32F32,
    pub y: I32F32,
}

#[derive(Component, Reflect, Deref, DerefMut)]
#[require(Position, Size)]
pub struct TileIndex(pub u32);

#[derive(Component, Reflect)]
pub struct TilePosition {
    x: u32,
    y: u32,
}

impl TilePosition {
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
    pub position: Position,
}

#[derive(Component, Reflect)]
#[require(ChunkStorage)]
pub struct TilemapStorage(Vec<Option<Entity>>);

impl TilemapStorage {
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
                Position::default(),
                Size::default(),
                Name::new("Tile"),
            ))
            .id();

        self.0[pos] = Some(tile);
    }
}

pub(crate) fn set_tiles_position(
    storages: Query<(&TileSize, &TilePadding), With<TilemapStorage>>,
    tiles: Query<
        (&mut Position, &mut Size, &TilePosition, &ChildOf),
        Or<(Changed<Position>, Changed<TilePosition>, Changed<Size>)>,
    >,
) {
    for (mut pos, mut size, tile_pos, parent) in tiles {
        let (tile_size, tile_padding) = storages.get(parent.0).unwrap();
        size.x = tile_size.x;
        size.y = tile_size.y;
        pos.x = I32F32::from_num(tile_pos.x) * (tile_size.x + tile_padding.x);
        pos.y = I32F32::from_num(tile_pos.y) * (tile_size.y + tile_padding.y);
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
        commands.entity(entity).insert(Collider);
    }
}

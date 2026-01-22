/*

use bevy::prelude::*;

use crate::{
    tilemap::{TilemapSize, TilemapStorage},
    transform::FixedTransform,
};

#[derive(Component, Reflect, Default)]
#[relationship_target(relationship = AttachedToChunk, linked_spawn)]
pub struct ChunkStorage(Vec<Entity>);

#[derive(Component, Reflect)]
#[relationship(relationship_target = ChunkStorage)]
pub struct AttachedToChunk(Entity);

#[derive(Component, Reflect)]
#[require(FixedTransform)]
pub struct Chunk {
    tiles: Vec<Entity>,
}

pub const CHUNK_W: u32 = 16;
pub const CHUNK_H: u32 = 16;

pub(crate) fn split_by_chunks(
    mut commands: Commands,
    query: Query<
        (Entity, &TilemapSize, &TilemapStorage, &mut ChunkStorage),
        Changed<TilemapStorage>,
    >,
    tile_data: Query<&FixedTransform>,
) {
    for (map, size, tiles, mut chunks) in query {
        chunks.0.clear();

        let map_w = size.width;
        let map_h = size.height;

        let chunk_rows = (map_h + CHUNK_H - 1) / CHUNK_H;
        let chunk_cols = (map_w + CHUNK_W - 1) / CHUNK_W;

        for cy in 0..chunk_rows {
            for cx in 0..chunk_cols {
                let mut set_pos = false;
                let mut transform = FixedTransform::default();
                let mut chunk = Chunk { tiles: vec![] };

                // границы чанка в тайлах
                let start_x = cx * CHUNK_W;
                let start_y = cy * CHUNK_H;

                let end_x = (start_x + CHUNK_W).min(map_w);
                let end_y = (start_y + CHUNK_H).min(map_h);

                // собираем тайлы внутри прямоугольника чанка
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let idx = (y * map_w + x) as usize;
                        if let Some(tile) = tiles.0[idx] {
                            let tile_transform = tile_data.get(tile).unwrap();
                            if !set_pos {
                                set_pos = true;
                                transform.position.x = tile_transform.position.x;
                                transform.position.y = tile_transform.position.y;
                            }

                            transform.size.x += tile_transform.size.x;
                            transform.size.y += tile_transform.size.y;
                            chunk.tiles.push(tile);
                        }
                    }
                }

                let chunk_entity = commands
                    .spawn((transform, chunk, Name::new("Chunk"), ChildOf(map)))
                    .id();
                chunks.0.push(chunk_entity);
            }
        }
    }
}
*/

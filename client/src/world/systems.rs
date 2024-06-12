use std::ops::Div;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::block_info::BlockInfoRegistry;
use crate::material::GlobalBlockAtlasMaterial;
use crate::player::Player;
use crate::world::chunk::{Chunk, CHUNK_SIZE, ChunkBlockData, ChunkPosition};

// TODO: figure out how to override yet-to-be-processed despawn events with newer spawn events and vice versa
// NOTE: could add instant::Instance to these events and before despawning, check if there's newer spawn event for the same position

// TODO: reconsider renaming "spawn/despawn" events to "load/unload" events
// NOTE: currently we got no world persistence, so we're just creating/destroying chunks on the fly

pub const WORLD_CHUNKS_HEIGHT: u32 = 1;
pub const WORLD_CHUNKS_LOAD_RADIUS: i32 = 5;
pub const WORLD_CHUNKS_LOAD_RADIUS_VERTICAL_MULTIPLIER: i32 = 2;
pub const WORLD_CHUNKS_LOAD_RADIUS_VERTICAL: i32 = WORLD_CHUNKS_LOAD_RADIUS * WORLD_CHUNKS_LOAD_RADIUS_VERTICAL_MULTIPLIER;

#[derive(Debug, Clone, Copy, Event)]
pub struct ChunkSpawn {
    chunk_position: IVec3,
}

impl ChunkSpawn {
    pub fn new(chunk_position: IVec3) -> Self {
        Self { chunk_position }
    }
}

#[derive(Debug, Clone, Copy, Event)]
pub struct ChunkDespawn {
    chunk_position: IVec3,
}

impl ChunkDespawn {
    pub fn new(chunk_position: IVec3) -> Self {
        Self { chunk_position }
    }
}

pub fn handle_spawn_chunk_events(
    mut commands: Commands,
    mut chunk_spawn_events: ResMut<Events<ChunkSpawn>>,
    // mut chunks: Query<(&mut Chunk, &ChunkPosition)>,
    mut meshes: ResMut<Assets<Mesh>>,
    atlas_material: Res<GlobalBlockAtlasMaterial>,
    block_info_registry: Res<BlockInfoRegistry>,
) {
    for event in chunk_spawn_events.drain() {
        // info!("Spawning chunk at position {:?}", event.chunk_position);

        let chunk = Chunk::new(event.chunk_position);

        chunk.spawn(
            &mut commands,
            &mut meshes,
            &atlas_material,
            &block_info_registry,
        );
    }
}

pub fn handle_despawn_chunk_events(
    mut commands: Commands,
    mut chunk_despawn_events: ResMut<Events<ChunkDespawn>>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks: Query<(Entity, &Handle<Mesh>, &ChunkPosition), With<ChunkBlockData>>,
) {
    for event in chunk_despawn_events.drain() {
        let Some((entity, mesh, _)) = chunks.iter().find(|(_, _, chunk_position)| event.chunk_position.eq(&chunk_position.0)) else {
            return;
        };

        // info!("Despawning chunk at position {:?}", event.chunk_position);

        meshes.remove(mesh);
        commands.entity(entity).despawn_recursive();
    }
}

fn is_chunk_in_radius(chunk_position: IVec3, player_chunk_position: IVec3) -> bool {
    let horizontal_distance = player_chunk_position.xz().distance_squared(chunk_position.xz());
    let vertical_distance = player_chunk_position.xz().distance_squared(chunk_position.xz());

    horizontal_distance <= WORLD_CHUNKS_LOAD_RADIUS || vertical_distance <= WORLD_CHUNKS_LOAD_RADIUS_VERTICAL
}

// Queues events to load & unload chunks around player's position
pub fn on_world_update(
    mut chunk_spawn_events: ResMut<Events<ChunkSpawn>>,
    mut chunk_despawn_events: ResMut<Events<ChunkDespawn>>,
    query_player: Query<&Transform, With<Player>>,
    query_loaded_chunks: Query<(Entity, &ChunkPosition), With<ChunkBlockData>>,
) {
    let Ok(player_transform) = query_player.get_single() else {
        return;
    };
    let player_position = player_transform.translation;
    let player_chunk_position = player_position.div(CHUNK_SIZE as f32).floor().as_ivec3();

    info!("---------------------");

    let (chunks_to_keep, chunks_to_unload): (Vec<_>, Vec<_>) = query_loaded_chunks
        .iter()
        .partition(|(_, chunk_position)| is_chunk_in_radius(chunk_position.0, player_chunk_position));

    if !chunks_to_unload.is_empty() {
        let unload_events = chunks_to_unload
            .iter()
            .map(|(_, chunk_position)| ChunkDespawn::new(chunk_position.0))
            .collect::<Vec<_>>();

        info!("Despawning {} chunks", unload_events.len());

        chunk_despawn_events.send_batch(unload_events);
    }

    let kept_chunks_coord_map = HashMap::from_iter(chunks_to_keep.iter().map(|(entity, chunk_position)| (chunk_position.0, true)));

    let from_y = (player_chunk_position.y - (WORLD_CHUNKS_LOAD_RADIUS_VERTICAL)).max(0);
    let to_y = (player_chunk_position.y + (WORLD_CHUNKS_LOAD_RADIUS_VERTICAL)).min(WORLD_CHUNKS_HEIGHT as i32 - 1);

    let from_x = player_chunk_position.x - (WORLD_CHUNKS_LOAD_RADIUS);
    let to_x = player_chunk_position.x + (WORLD_CHUNKS_LOAD_RADIUS);

    let from_z = player_chunk_position.z - (WORLD_CHUNKS_LOAD_RADIUS);
    let to_z = player_chunk_position.z + (WORLD_CHUNKS_LOAD_RADIUS);

    let mut load_chunk_events = vec![];

    for y in from_y..=to_y {
        for x in from_x..=to_x {
            for z in from_z..=to_z {
                let chunk_position = IVec3::new(x, y, z);

                if !is_chunk_in_radius(chunk_position, player_chunk_position) {
                    continue;
                }

                if kept_chunks_coord_map.contains_key(&chunk_position) {
                    continue;
                }

                load_chunk_events.push(ChunkSpawn::new(chunk_position));
            }
        }
    }

    if !load_chunk_events.is_empty() {
        info!("Spawning {} chunks", load_chunk_events.len());

        chunk_spawn_events.send_batch(load_chunk_events);
    }
}

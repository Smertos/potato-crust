use bevy::prelude::*;
use crate::assets::AppState;
use crate::world::systems::{ChunkDespawn, ChunkSpawn, handle_despawn_chunk_events, handle_spawn_chunk_events, on_world_update};

pub mod chunk;
pub mod voxel;
mod block;
mod systems;


pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkSpawn>()
            .add_event::<ChunkDespawn>()
            .add_systems(Update, (on_world_update, handle_despawn_chunk_events, handle_spawn_chunk_events).chain().run_if(in_state(AppState::InGame)));
    }
}
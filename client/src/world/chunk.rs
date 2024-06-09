use std::sync::Arc;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use crate::block_info::{BlockInfoRegistry, BlockSide};
use crate::material::ATTRIBUTE_ATLAS_TEXTURE_INDEX;
use crate::world::block::Block;
use crate::world::voxel::BlockVoxel;

pub const CHUNK_SIZE: u32 = 16;
pub const CHUNK_SIZE_OUTER: u32 = CHUNK_SIZE + 2;
pub const WORLD_CHUNKS_HEIGHT: u32 = 16;

pub type BlockData = [Option<Arc<Block>>; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize];
pub type ChunkVoxelShape = ConstShape3u32<CHUNK_SIZE_OUTER, CHUNK_SIZE_OUTER, CHUNK_SIZE_OUTER>;

#[derive(Clone, Debug)]
pub struct Chunk {
    block_data: BlockData,
    voxels: [BlockVoxel; CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize],
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_block(&self, position: Vec3) -> Option<Arc<Block>> {
        let block_position = position.floor().as_ivec3();
        let block_index = block_position.x + block_position.y * CHUNK_SIZE as i32 + block_position.z * CHUNK_SIZE as i32 * CHUNK_SIZE as i32;

        self.block_data[block_index as usize].clone()
    }

    // TODO: update voxels from blocks
    pub fn update_voxels(&mut self, block_info_registry: &BlockInfoRegistry) {
        let mut voxels = [BlockVoxel::AIR; CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize];

        for z in 1..CHUNK_SIZE_OUTER - 1 {
            for y in 1..CHUNK_SIZE_OUTER - 1 {
                for x in 1..CHUNK_SIZE_OUTER - 1 {
                    let i = ChunkVoxelShape::linearize([x, y, z]);
                    let block_info = match y {
                        _ if y == (CHUNK_SIZE_OUTER - 2) => block_info_registry.get_block_info("potato_crust:grass"),
                        _ if y < (CHUNK_SIZE_OUTER - 2) && y > (CHUNK_SIZE_OUTER - 5) => block_info_registry.get_block_info("potato_crust:dirt"),
                        _ => block_info_registry.get_block_info("potato_crust:cobblestone"),
                    };

                    voxels[i as usize] = BlockVoxel {
                        block_info_key_hash: block_info.get_registry_name_hash(),
                        is_air: false,
                        is_translucent: false,
                    };
                }
            }
        }

        self.voxels = voxels;
    }

    pub fn serialize_voxels_to_render_mesh(&self, block_info_registry: &BlockInfoRegistry) -> Mesh {
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let mut buffer = GreedyQuadsBuffer::new(self.voxels.len());
        // let mut buffer = UnitQuadBuffer::new();
        greedy_quads(
            &self.voxels,
            &ChunkVoxelShape {},
            [0; 3],
            [CHUNK_SIZE_OUTER - 1; 3],
            &faces,
            &mut buffer,
        );
        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;

        // visible_block_faces(
        //     &voxels,
        //     &SampleShape {},
        //     [0; 3],
        //     [CHUNK_SIZE_OUTER - 1; 3],
        //     &faces,
        //     &mut buffer,
        // );
        // let num_indices = buffer.num_quads() * 6;
        // let num_vertices = buffer.num_quads() * 4;

        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        let mut tex_coords = Vec::with_capacity(num_vertices);
        let mut atlas_texture_indices = Vec::with_capacity(num_vertices);

        // info!("Total number of quads: {}", buffer.num_quads());
        info!("Total number of quads: {}", buffer.quads.num_quads());

        // for (group, face) in buffer.groups.into_iter().zip(faces.into_iter()) {
        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
                normals.extend_from_slice(&face.quad_mesh_normals());
                tex_coords.extend_from_slice(&face.tex_coords(
                    RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                    true,
                    &quad.into(),
                ));

                let normals: Vec3 = face.quad_mesh_normals()[0].into();
                let block_side = BlockSide::match_normal_vector(normals);
                let block_info = block_info_registry.get_block_info_by_hash(quad.voxel.block_info_key_hash);
                let block_texture_id = block_info.get_side_texture_id(block_side).unwrap_or(255);

                atlas_texture_indices.extend_from_slice(&[
                    block_texture_id,
                    block_texture_id,
                    block_texture_id,
                    block_texture_id,
                ]);
            }
        }
        // Center the mesh.
        for p in &mut positions {
            *p = (Vec3::from(*p) - Vec3::splat(10.0)).into();
        }

        let mut render_mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        dbg!(&tex_coords);

        render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
        render_mesh.insert_attribute(ATTRIBUTE_ATLAS_TEXTURE_INDEX, atlas_texture_indices);
        render_mesh.insert_indices(Indices::U32(indices));

        render_mesh
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            block_data: core::array::from_fn(|_i| None),
            voxels: [BlockVoxel::AIR; CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize],
        }
    }
}

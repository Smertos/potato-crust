use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};

use crate::block_info::{BlockInfoRegistry, BlockSide};
use crate::material::{ATTRIBUTE_ATLAS_TEXTURE_INDEX, BlockAtlasPbrBundle, GlobalBlockAtlasMaterial};
use crate::world::block::Block;
use crate::world::voxel::BlockVoxel;

pub const CHUNK_SIZE: u32 = 16;
pub const CHUNK_SIZE_OUTER: u32 = CHUNK_SIZE + 2;

pub type ChunkVoxelShape = ConstShape3u32<CHUNK_SIZE_OUTER, CHUNK_SIZE_OUTER, CHUNK_SIZE_OUTER>;

#[derive(Clone, Component, Debug)]
pub struct ChunkBlockData(pub [Option<Arc<Block>>; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize]);

impl Default for ChunkBlockData {
    fn default() -> Self {
        Self(core::array::from_fn(|_i| None))
    }
}

#[derive(Clone, Component, Debug)]
pub struct ChunkVoxelData(pub [BlockVoxel; CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize]);

impl Default for ChunkVoxelData {
    fn default() -> Self {
        Self([BlockVoxel::default(); CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize * CHUNK_SIZE_OUTER as usize])
    }
}

#[derive(Component, Clone, Debug, Default, Eq, PartialEq)]
pub struct ChunkPosition(pub IVec3);


#[derive(Bundle, Clone, Debug, Default)]
pub struct Chunk {
    block_data: ChunkBlockData,
    position: ChunkPosition,
    voxels: ChunkVoxelData,
}

impl Chunk {
    pub fn new(position: IVec3) -> Self {
        Self {
            position: ChunkPosition(position),
            ..Default::default()
        }
    }

    pub fn spawn(mut self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, atlas_material: &GlobalBlockAtlasMaterial, block_info_registry: &BlockInfoRegistry) {
        self.update_voxels(block_info_registry);

        let chunk_mesh = self.serialize_voxels_to_render_mesh(block_info_registry);

        commands.spawn((self, BlockAtlasPbrBundle {
            mesh: meshes.add(chunk_mesh),
            material: atlas_material.0.clone(),
            ..Default::default()
        }));
    }

    pub fn get_block(&self, position: Vec3) -> Option<Arc<Block>> {
        let block_position = position.floor().as_ivec3();
        let block_index = block_position.x + block_position.y * CHUNK_SIZE as i32 + block_position.z * CHUNK_SIZE as i32 * CHUNK_SIZE as i32;

        self.block_data.0[block_index as usize].clone()
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

        self.voxels.0 = voxels;
    }

    pub fn serialize_voxels_to_render_mesh(&self, block_info_registry: &BlockInfoRegistry) -> Mesh {
        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

        let position_offset = self.position.0 * CHUNK_SIZE as i32;

        let mut buffer = GreedyQuadsBuffer::new(self.voxels.0.len());
        // let mut buffer = UnitQuadBuffer::new();
        greedy_quads(
            &self.voxels.0,
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
        // info!("Total number of quads: {}", buffer.quads.num_quads());

        // for (group, face) in buffer.groups.into_iter().zip(faces.into_iter()) {
        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                let voxel_position = &mut face.quad_mesh_positions(&quad.into(), 1.0);

                for vertex in voxel_position.iter_mut() {
                    vertex[0] += position_offset.x as f32;
                    vertex[1] += position_offset.y as f32;
                    vertex[2] += position_offset.z as f32;
                }

                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(voxel_position.as_slice());
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

        render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        render_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
        render_mesh.insert_attribute(ATTRIBUTE_ATLAS_TEXTURE_INDEX, atlas_texture_indices);
        render_mesh.insert_indices(Indices::U32(indices));

        render_mesh
    }
}
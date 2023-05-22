use bevy::prelude::*;
use block_mesh::{greedy_quads, GreedyQuadsBuffer, MergeVoxel, QuadCoordinateConfig, RIGHT_HANDED_Y_UP_CONFIG, Voxel, VoxelVisibility};
use block_mesh::ndshape::ConstShape3u32;

const CHUNK_SIZE_BLOCKS: u32 = 16;
const CHUNK_SIZE: u32 = CHUNK_SIZE_BLOCKS + 2; // 2 blocks of padding
const CHUNK_CUBES_ARRAY_SIZE: usize = (CHUNK_SIZE_BLOCKS * CHUNK_SIZE_BLOCKS * CHUNK_SIZE_BLOCKS) as usize;
const CHUNK_CUBES_BUFFER_SIZE: usize = (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize;

type ChunkShape = ConstShape3u32<CHUNK_SIZE, CHUNK_SIZE, CHUNK_SIZE>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChunkBlock {
    id: u32,
}

impl Voxel for ChunkBlock {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.id == 0 {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for ChunkBlock {
    type MergeValue = u32;

    fn merge_value(&self) -> Self::MergeValue {
        self.id
    }
}


#[derive(Component)]
pub struct Chunk {
    cubes: [ChunkBlock; CHUNK_CUBES_ARRAY_SIZE],
    vertices: Vec<[f32; 3]>,
    indices: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

impl Chunk {
    const VOXEL_SIZE: f32 = 1.0;
    const VERTICES_PER_QUAD: usize = 4;
    const INDICES_PER_QUAD: usize = 6;
    const NORMALS_PER_QUAD: usize = 4;
    const UVS_PER_QUAD: usize = 4;

    fn new() -> Self {
        Self {
            cubes: [ChunkBlock { id: 0, }; CHUNK_CUBES_ARRAY_SIZE],
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
        }
    }

    fn greedy_mesh(&mut self) {
        let QuadCoordinateConfig { faces, u_flip_face, .. } = RIGHT_HANDED_Y_UP_CONFIG;
        let mut buffer = GreedyQuadsBuffer::new(CHUNK_CUBES_BUFFER_SIZE);

        greedy_quads(&self.cubes, &ChunkShape {}, [0; 3], [CHUNK_SIZE - 1; 3], &faces, &mut buffer);

        let num_quads = buffer.quads.num_quads();

        self.vertices = Vec::with_capacity(num_quads * Self::VERTICES_PER_QUAD);
        self.indices = Vec::with_capacity(num_quads * Self::INDICES_PER_QUAD);
        self.normals = Vec::with_capacity(num_quads * Self::NORMALS_PER_QUAD);
        self.uvs = Vec::with_capacity(num_quads * Self::UVS_PER_QUAD);

        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                self.vertices.extend_from_slice(&face.quad_mesh_positions(&quad, Self::VOXEL_SIZE));
                self.indices.extend_from_slice(&face.quad_mesh_indices(self.vertices.len() as u32));
                self.normals.extend_from_slice(&face.quad_mesh_normals());
                self.uvs.extend_from_slice(&face.tex_coords(u_flip_face, true, &quad));
            }
        }
    }
}
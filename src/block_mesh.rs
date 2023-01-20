use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::utils::HashMap;
use bitmask_enum::bitmask;
use rayon::prelude::*;

type MeshVertex = ([f32; 3], [f32; 3], [f32; 2]);

#[bitmask(u8)]
pub enum BlockSide {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl BlockSide {
    pub fn all_sides() -> Self {
        BlockSide::Front
            | BlockSide::Back
            | BlockSide::Left
            | BlockSide::Right
            | BlockSide::Top
            | BlockSide::Bottom
    }
}

#[derive(Debug, Resource, TypeUuid)]
#[uuid = "edd30cca-c0fc-44e4-ac99-0411af842f04"]
pub struct BlockMeshStorage {
    meshes: HashMap<u8, Handle<Mesh>>,
}

impl BlockMeshStorage {
    const VERTICES: [MeshVertex; 4 * 6] = [
        // Front
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
        ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
        // Back
        ([1.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
        ([0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
        ([1.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
        // Left
        ([0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
        ([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        ([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
        ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
        // Right
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
        ([1.0, 0.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
        ([1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
        // Top
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
        // Bottom
        ([0.0, 0.0, 1.0], [0.0, -1.0, 0.0], [0.0, 1.0]),
        ([1.0, 0.0, 1.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
        ([1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
        ([0.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0]),
    ];
    const FRONT_INDICES: [u16; 6] = [1, 0, 2, 2, 0, 3];
    const BACK_INDICES: [u16; 6] = [5, 4, 6, 6, 4, 7];
    const LEFT_INDICES: [u16; 6] = [9, 8, 10, 10, 8, 11];
    const RIGHT_INDICES: [u16; 6] = [13, 12, 14, 14, 12, 15];
    const TOP_INDICES: [u16; 6] = [17, 16, 18, 18, 16, 19];
    const BOTTOM_INDICES: [u16; 6] = [21, 20, 22, 22, 20, 23];

    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
        }
    }

    pub fn generate_meshes(&mut self, mesh_assets: &mut ResMut<Assets<Mesh>>) {
        let mask_min: u8 = BlockSide::Front.bits();
        let mask_max: u8 = BlockSide::all_sides().bits();

        let meshes = (mask_min..=mask_max)
            .into_par_iter()
            .map(|x| {
                let bits: BlockSide = x.into();

                (x, Self::generate_mesh(bits))
            })
            .collect::<Vec<(u8, Mesh)>>();

        for (key, mesh) in meshes {
            let mesh_handle = mesh_assets.add(mesh);

            self.meshes.insert(key, mesh_handle);
        }

        debug!("Generated and stored {} variations of block mesh", mask_max);
    }

    fn generate_mesh(mask: BlockSide) -> Mesh {
        let has_front_side: bool = mask.contains(BlockSide::Front);
        let has_back_side: bool = mask.contains(BlockSide::Back);
        let has_left_side: bool = mask.contains(BlockSide::Left);
        let has_right_side: bool = mask.contains(BlockSide::Right);
        let has_top_side: bool = mask.contains(BlockSide::Top);
        let has_bottom_side: bool = mask.contains(BlockSide::Bottom);

        let mut indices: Vec<u16> = Vec::new();

        if has_front_side {
            for index in Self::FRONT_INDICES {
                indices.push(index);
            }
        }

        if has_back_side {
            for index in Self::BACK_INDICES {
                indices.push(index);
            }
        }

        if has_left_side {
            for index in Self::LEFT_INDICES {
                indices.push(index);
            }
        }

        if has_right_side {
            for index in Self::RIGHT_INDICES {
                indices.push(index);
            }
        }

        if has_top_side {
            for index in Self::TOP_INDICES {
                indices.push(index);
            }
        }

        if has_bottom_side {
            for index in Self::BOTTOM_INDICES {
                indices.push(index);
            }
        }

        let positions: Vec<_> = Self::VERTICES.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = Self::VERTICES.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = Self::VERTICES.iter().map(|(_, _, uv)| *uv).collect();

        let indices = Indices::U16(indices);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }

    #[allow(dead_code)]
    pub fn get_mesh(&self, sides: BlockSide) -> Option<Handle<Mesh>> {
        let bits: u8 = sides.bits();

        self.meshes
            .get(&bits)
            .map(|handle_ref| handle_ref.clone_weak())
    }

    #[allow(dead_code)]
    pub fn get_full(&self) -> Option<Handle<Mesh>> {
        let sides = BlockSide::all_sides();

        self.get_mesh(sides)
    }
}

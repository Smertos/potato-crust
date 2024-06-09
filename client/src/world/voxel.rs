use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

/// Basic voxel type with one byte of texture layers
#[derive(Copy, Clone, Debug)]
pub struct BlockVoxel {
    pub block_info_key_hash: u64,
    pub is_air: bool,
    pub is_translucent: bool,
}

impl BlockVoxel {
    pub const AIR: Self = Self {
        block_info_key_hash: 0,
        is_air: true,
        is_translucent: true,
    };
}

impl Default for BlockVoxel {
    fn default() -> Self {
        Self::AIR
    }
}

impl MergeVoxel for BlockVoxel {
    type MergeValue = (bool, bool, u64);
    type MergeValueFacingNeighbour = (bool, bool, u64);

    fn merge_value(&self) -> Self::MergeValue {
        // (self.is_air as u16 + 1) * (self.block_atlas_id as u16 + 1)
        (self.is_air, self.is_translucent, self.block_info_key_hash)
    }

    fn merge_value_facing_neighbour(&self) -> Self::MergeValueFacingNeighbour {
        // (self.is_air as u16 + 1) * (self.block_atlas_id as u16 + 1)
        (self.is_air, self.is_translucent, self.block_info_key_hash)
    }
}

impl Voxel for BlockVoxel {
    fn get_visibility(&self) -> VoxelVisibility {
        match (self.is_air, self.is_translucent, self.block_info_key_hash) {
            (true, _, _) => VoxelVisibility::Empty,
            (_, _, 0) => VoxelVisibility::Empty,
            (false, true, _) => VoxelVisibility::Translucent,
            _ => VoxelVisibility::Opaque,
        }
    }
}
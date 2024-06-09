use std::sync::Arc;
use bevy::math::Vec3;
use crate::block_info::BlockInfo;
use crate::world::voxel::BlockVoxel;

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub info: Option<Arc<BlockInfo>>,
    pub position: Vec3,
}

impl From<Block> for BlockVoxel {
    fn from(other: Block) -> BlockVoxel {
        match other.info {
            Some(info) => BlockVoxel {
                block_info_key_hash: info.get_registry_name_hash(),
                is_air: false,
                is_translucent: info.is_translucent,
            },
            None => BlockVoxel::AIR,
        }
    }
}

impl From<Option<Block>> for BlockVoxel {
    fn from(other: Option<Block>) -> BlockVoxel {
        match other {
            Some(block) => BlockVoxel::from(block),
            None => BlockVoxel {
                block_info_key_hash: 0,
                is_air: true,
                is_translucent: true,
            },
        }
    }
}


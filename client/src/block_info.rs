use std::sync::Arc;

use bevy::prelude::*;
use color_eyre::eyre::eyre;
use dashmap::DashMap;
use fasthash::city;
use strum::IntoEnumIterator;

use crate::assets::{
    AppState, ATLAS_TEXTURE_COBBLESTONE, ATLAS_TEXTURE_DIRT, ATLAS_TEXTURE_GRASS_SIDE,
    ATLAS_TEXTURE_GRASS_TOP,
};

pub const BLOCK_DIRT: BlockInfo = BlockInfo {
    category: None,
    is_translucent: false,
    name: "dirt",
    sides: BlockSides::new_for_all(ATLAS_TEXTURE_DIRT),
};

pub const BLOCK_GRASS: BlockInfo = BlockInfo {
    category: None,
    is_translucent: false,
    name: "grass",
    sides: BlockSides {
        all: Some(ATLAS_TEXTURE_GRASS_SIDE),
        top: Some(ATLAS_TEXTURE_GRASS_TOP),
        bottom: Some(ATLAS_TEXTURE_DIRT),
        left: None,
        right: None,
        front: None,
        back: None,
    },
};

pub const BLOCK_COBBLESTONE: BlockInfo = BlockInfo {
    category: None,
    is_translucent: false,
    name: "cobblestone",
    sides: BlockSides::new_for_all(ATLAS_TEXTURE_COBBLESTONE),
};

#[derive(strum::EnumIter)]
pub enum BlockSide {
    Front = 0,
    Back = 1,
    Left = 2,
    Right = 3,
    Top = 4,
    Bottom = 5,
}

impl BlockSide {
    pub fn match_normal_vector(normal: Vec3) -> BlockSide {
        let mut best_side = None;
        let mut best_dot = 0.0;

        for side in BlockSide::iter() {
            let side_normal = match side {
                BlockSide::Front => Vec3::Z,
                BlockSide::Back => -Vec3::Z,
                BlockSide::Left => -Vec3::X,
                BlockSide::Right => Vec3::X,
                BlockSide::Top => Vec3::Y,
                BlockSide::Bottom => -Vec3::Y,
            };

            let dot = normal.dot(side_normal);

            if dot > best_dot {
                best_side = Some(side);
                best_dot = dot;
            }
        }

        best_side.unwrap_or(BlockSide::Front)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BlockSides {
    pub all: Option<u32>,
    pub front: Option<u32>,
    pub back: Option<u32>,
    pub left: Option<u32>,
    pub right: Option<u32>,
    pub top: Option<u32>,
    pub bottom: Option<u32>,
}

impl BlockSides {
    pub fn get_side_texture_id(&self, side: BlockSide) -> Option<u32> {
        match side {
            BlockSide::Front => self.front.or(self.all),
            BlockSide::Back => self.back.or(self.all),
            BlockSide::Left => self.left.or(self.all),
            BlockSide::Right => self.right.or(self.all),
            BlockSide::Top => self.top.or(self.all),
            BlockSide::Bottom => self.bottom.or(self.all),
        }
    }
}

impl BlockSides {
    pub const fn new_for_all(texture_id: u32) -> Self {
        Self {
            all: Some(texture_id),
            front: None,
            back: None,
            left: None,
            right: None,
            top: None,
            bottom: None,
        }
    }

    pub const fn new_for_side(
        side: BlockSide,
        side_texture_id: u32,
        fallback_texture_id: u32,
    ) -> Self {
        match side {
            BlockSide::Front => Self {
                all: Some(fallback_texture_id),
                front: Some(side_texture_id),
                back: None,
                left: None,
                right: None,
                top: None,
                bottom: None,
            },
            BlockSide::Back => Self {
                all: Some(fallback_texture_id),
                front: None,
                back: Some(side_texture_id),
                left: None,
                right: None,
                top: None,
                bottom: None,
            },
            BlockSide::Left => Self {
                all: Some(fallback_texture_id),
                front: None,
                back: None,
                left: Some(side_texture_id),
                right: None,
                top: None,
                bottom: None,
            },
            BlockSide::Right => Self {
                all: Some(fallback_texture_id),
                front: None,
                back: None,
                left: None,
                right: Some(side_texture_id),
                top: None,
                bottom: None,
            },
            BlockSide::Top => Self {
                all: Some(fallback_texture_id),
                front: None,
                back: None,
                left: None,
                right: None,
                top: Some(side_texture_id),
                bottom: None,
            },
            BlockSide::Bottom => Self {
                all: Some(fallback_texture_id),
                front: None,
                back: None,
                left: None,
                right: None,
                top: None,
                bottom: Some(side_texture_id),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BlockInfo {
    pub category: Option<&'static str>,
    pub name: &'static str,
    pub is_translucent: bool,
    pub sides: BlockSides,
}

impl BlockInfo {
    pub fn get_registry_name(&self) -> String {
        match self.category {
            Some(ref category) => format!("{}:{}", category, &self.name),
            None => self.name.to_string(),
        }
    }

    pub fn get_registry_name_hash(&self) -> u64 {
        let registry_name = self.get_registry_name();
        city::hash64(registry_name.as_bytes())
    }

    pub fn get_side_texture_id(&self, side: BlockSide) -> Option<u32> {
        self.sides.get_side_texture_id(side)
    }
}

impl PartialEq<Self> for BlockInfo {
    fn eq(&self, other: &Self) -> bool {
        self.get_registry_name() == other.get_registry_name()
    }
}

impl Eq for BlockInfo {}

#[derive(Clone, Default, Resource)]
pub struct BlockInfoRegistry {
    block_map: Arc<DashMap<u64, Arc<BlockInfo>>>,
    reverse_key_map: Arc<DashMap<u64, String>>,
}

impl BlockInfoRegistry {
    pub(crate) fn initialize() -> color_eyre::Result<Self> {
        let mut block_info_registry = BlockInfoRegistry::default();

        block_info_registry.register("potato_crust", BLOCK_DIRT)?;
        block_info_registry.register("potato_crust", BLOCK_GRASS)?;
        block_info_registry.register("potato_crust", BLOCK_COBBLESTONE)?;

        Ok(block_info_registry)
    }

    pub fn get_block_info(&self, registry_name: impl Into<String>) -> Arc<BlockInfo> {
        let registry_name = registry_name.into();
        let key_hash = city::hash64(registry_name.as_bytes());

        self.block_map
            .get(&key_hash)
            .map(|x| x.clone())
            .expect(format!("block info with name `{}` not found", registry_name).as_str())
    }

    pub fn get_block_info_by_hash(&self, key_hash: u64) -> Arc<BlockInfo> {
        self.block_map
            .get(&key_hash)
            .map(|x| x.clone())
            .expect(format!("block info with hash `{}` not found", key_hash).as_str())
    }

    pub fn register(
        &mut self,
        category: &'static str,
        mut block_info: BlockInfo,
    ) -> color_eyre::Result<String> {
        let registry_name = format!("{}:{}", category, &block_info.name);
        let key_hash = city::hash64(registry_name.as_bytes());

        if self.block_map.contains_key(&key_hash) {
            return Err(eyre!(
                "BlockInfoRegistry::register: block info with name `{}` already exists",
                registry_name
            ));
        }

        block_info.category = Some(category);

        self.block_map.insert(key_hash, Arc::new(block_info));
        self.reverse_key_map.insert(key_hash, registry_name.clone());

        Ok(registry_name)
    }
}

pub fn initialize_block_info_registry(mut commands: Commands) {
    let block_info_registry =
        BlockInfoRegistry::initialize().expect("Failed to initialize block info registry");

    commands.insert_resource(block_info_registry);
}

#[derive(Default)]
pub struct BlockInfoPlugin;

impl Plugin for BlockInfoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockInfoRegistry>().add_systems(
            OnEnter(AppState::LoadingAssets),
            initialize_block_info_registry,
        );
    }
}

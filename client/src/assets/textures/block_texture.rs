use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::material::block_material::BlockMaterial;

#[derive(Component, Clone)]
pub struct BlockTextureName(pub String);

/// Named block texture
#[derive(Clone, Resource, TypeUuid)]
#[uuid = "465c1456-527c-4ac9-a414-ede7d5b9b635"]
pub struct BlockTexture {
    pub name: BlockTextureName,
    // NOTE: only for testing purposes, will be removed later
    pub material: Handle<BlockMaterial>,
    pub texture_image: Handle<Image>,
}

impl BlockTexture {
    pub fn new(
        name: impl Into<String>,
        material: Handle<BlockMaterial>,
        texture_image: Handle<Image>,
    ) -> Self {
        Self {
            name: BlockTextureName(name.into()),
            material,
            texture_image,
        }
    }
}

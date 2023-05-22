use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};

#[allow(dead_code)]
#[derive(AsBindGroup, Resource, TypeUuid, Clone)]
#[uuid = "de62a8d7-de20-4c39-9e98-a319f9cff438"]
pub struct BlockAtlasMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas_texture: Handle<Image>,
}

impl BlockAtlasMaterial {
    pub fn new(atlas: &TextureAtlas) -> Self {
        Self {
            atlas_texture: atlas.texture.clone(),
        }
    }
}

impl Material for BlockAtlasMaterial {}

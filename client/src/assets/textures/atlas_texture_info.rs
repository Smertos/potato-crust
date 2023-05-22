use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(Debug, Clone, Resource, TypeUuid)]
#[uuid = "de62a8d7-de20-4c39-1e64-a319f9cff438"]
pub struct AtlasTextureInfo {
    atlas: Handle<TextureAtlas>,
    name: String,
    index: usize,
}

impl AtlasTextureInfo {
    pub fn new(atlas: &Handle<TextureAtlas>, name: &str, index: usize) -> Self {
        Self {
            atlas: atlas.clone(),
            name: name.to_owned(),
            index,
        }
    }

    pub fn get_rect(&self, atlases: &Res<Assets<TextureAtlas>>) -> (Vec2, Vec2) {
        let atlas = atlases
            .get(&self.atlas)
            .expect("AtlasTextureInfo::get_rect: atlas not found");

        atlas
            .textures
            .get(self.index)
            .map(|rect| (rect.min, rect.max))
            .expect("AtlasTextureInfo::get_rect: texture index not found in atlas")
    }
}

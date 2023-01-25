use crate::block_texture::BlockTexture;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

#[derive(Clone, Resource, TypeUuid)]
#[uuid = "2c73ad47-e339-4d83-b182-afa17c929eb8"]
pub struct AtlasManager {
    atlases: HashMap<u32, Vec<Handle<TextureAtlas>>>,
}

impl AtlasManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            atlases: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn build_category_atlases(
        &mut self,
        side_size: u32,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        mut images: &mut ResMut<Assets<Image>>,
        texture_images: Vec<&BlockTexture>,
    ) {
        const ATLAS_SIDE_SIZE: usize = 16;
        const ATLAS_TOTAL_SIZE: usize = ATLAS_SIDE_SIZE * ATLAS_SIDE_SIZE;

        debug!(
            "{0} textures selected for atlases sized {1}x{1}",
            texture_images.len(),
            side_size
        );

        let atlas_size = ATLAS_SIDE_SIZE as f32 * side_size as f32;
        let atlas_size: Vec2 = Vec2::new(atlas_size, atlas_size);

        for texture_chunk in texture_images.chunks(ATLAS_TOTAL_SIZE) {
            let mut builder = TextureAtlasBuilder::default()
                .initial_size(atlas_size)
                .max_size(atlas_size);

            for texture in texture_chunk {
                let Some(texture_image) = images.get(&texture.texture_image) else {
                    warn!("Texture image for handle {:?} could not be found", &texture.texture_image.id());
                    continue;
                };

                builder.add_texture(texture.texture_image.clone(), texture_image);
            }

            match builder.finish(&mut images) {
                Ok(atlas) => {
                    let atlas_handle = texture_atlases.add(atlas);

                    match self.atlases.get_mut(&side_size) {
                        // TODO: perhaps unload all used textures
                        Some(atlas_list) => atlas_list.push(atlas_handle),
                        None => {
                            self.atlases.insert(side_size, vec![atlas_handle]);
                        }
                    };
                }
                Err(err) => error!("Failed to build texture atlas: {}", err),
            };
        }
    }

    #[allow(dead_code)]
    pub fn process_textures(
        &mut self,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        images: &mut ResMut<Assets<Image>>,
        block_textures: &Res<Assets<BlockTexture>>,
    ) {
        let mut categorized_textures = HashMap::<u32, Vec<&BlockTexture>>::new();

        for (_, block_texture) in block_textures.iter() {
            let Some(texture_image) = images.get(&block_texture.texture_image) else {
                warn!("Texture image for handle {:?} could not be found", &block_texture.texture_image.id());
                continue;
            };
            let texture_extend = texture_image.texture_descriptor.size;
            let (texture_width, texture_height) = (texture_extend.width, texture_extend.height);
            let texture_size = u32::max(texture_width, texture_height);

            match categorized_textures.get_mut(&texture_size) {
                Some(category_list) => category_list.push(block_texture),
                None => {
                    categorized_textures.insert(texture_size, vec![block_texture]);
                }
            };
        }

        for (side_size, texture_images) in categorized_textures.iter() {
            self.build_category_atlases(
                side_size.clone(),
                texture_atlases,
                images,
                texture_images.to_vec(),
            );
        }
    }
}

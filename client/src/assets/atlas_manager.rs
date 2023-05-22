use crate::assets::registry::AtlasTextureInfoRegistry;
use crate::assets::textures::atlas_texture_info::AtlasTextureInfo;
use crate::material::block_atlas_material::BlockAtlasMaterial;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

use super::textures::block_texture::BlockTexture;

#[derive(Clone, Resource, TypeUuid)]
#[uuid = "2c73ad47-e339-4d83-b182-afa17c929eb8"]
pub struct AtlasManager {
    atlases: HashMap<u32, Handle<TextureAtlas>>,
}

impl AtlasManager {
    pub fn new() -> Self {
        Self {
            atlases: HashMap::new(),
        }
    }

    fn build_atlas_textures(
        &self,
        commands: &mut Commands,
        atlas_texture_info_registry: &mut ResMut<AtlasTextureInfoRegistry>,
        atlas_texture_infos: &mut ResMut<Assets<AtlasTextureInfo>>,
        atlas: &TextureAtlas,
        atlas_handle: &Handle<TextureAtlas>,
        texture_images: Vec<&BlockTexture>,
    ) {
        commands.insert_resource(BlockAtlasMaterial::new(atlas));

        for (_, block_texture) in texture_images.iter().enumerate() {
            let index = atlas
                .get_texture_index(&block_texture.texture_image)
                .expect("must contain expected texture");

            let atlas_texture_info =
                AtlasTextureInfo::new(&atlas_handle, &block_texture.name.0, index);
            let atlas_texture_info_handle = atlas_texture_infos.add(atlas_texture_info);

            atlas_texture_info_registry
                .insert(&block_texture.name.0, atlas_texture_info_handle)
                .expect("could not insert atlas texture info into registry");
        }
    }

    fn build_category_atlases(
        &mut self,
        commands: &mut Commands,
        texture_side_size: u32,
        atlas_texture_info_registry: &mut ResMut<AtlasTextureInfoRegistry>,
        atlas_texture_infos: &mut ResMut<Assets<AtlasTextureInfo>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
        mut images: &mut ResMut<Assets<Image>>,
        texture_images: Vec<&BlockTexture>,
    ) {
        let mut atlas_axis_amount: usize = 32;
        let mut atlas_total_amount: usize = atlas_axis_amount * atlas_axis_amount;

        while texture_images.len() > atlas_total_amount {
            atlas_axis_amount *= 2;
            atlas_total_amount = atlas_axis_amount * atlas_axis_amount;
        }

        debug!(
            "{0} textures selected for atlases sized {1}x{1}",
            texture_images.len(),
            texture_side_size
        );

        let atlas_size = atlas_axis_amount as f32 * texture_side_size as f32;
        let atlas_size: Vec2 = Vec2::new(atlas_size, atlas_size);

        let mut builder = TextureAtlasBuilder::default()
            .initial_size(atlas_size)
            .max_size(atlas_size);

        for texture in &texture_images {
            let Some(texture_image) = images.get(&texture.texture_image) else {
                warn!("Texture image for handle {:?} could not be found", &texture.name.0);
                continue;
            };

            debug!("adding texture to atlas: {}", &texture.name.0);

            builder.add_texture(texture.texture_image.clone(), texture_image);
        }

        match builder.finish(&mut images) {
            Ok(atlas) => {
                let atlas_copy = atlas.clone();
                let atlas_handle = texture_atlases.add(atlas);

                self.build_atlas_textures(
                    commands,
                    atlas_texture_info_registry,
                    atlas_texture_infos,
                    &atlas_copy,
                    &atlas_handle,
                    texture_images,
                );

                self.atlases.insert(texture_side_size, atlas_handle);
            }
            Err(err) => error!("Failed to build texture atlas: {}", err),
        };
    }

    pub fn process_textures(
        &mut self,
        mut commands: Commands,
        atlas_texture_info_registry: &mut ResMut<AtlasTextureInfoRegistry>,
        atlas_texture_infos: &mut ResMut<Assets<AtlasTextureInfo>>,
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
                &mut commands,
                side_size.clone(),
                atlas_texture_info_registry,
                atlas_texture_infos,
                texture_atlases,
                images,
                texture_images.to_vec(),
            );
        }
    }
}

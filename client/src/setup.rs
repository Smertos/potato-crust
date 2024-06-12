use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::texture::{
    ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor,
};

use crate::assets::{AppState, BlockTextureAtlasImage};
use crate::material::{BlockAtlasMaterial, GlobalBlockAtlasMaterial};
use crate::player::PlayerBundle;

pub fn setup(
    mut commands: Commands,
    atlas_image_handle: Res<BlockTextureAtlasImage>,
    mut atlas_materials: ResMut<Assets<BlockAtlasMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = true;
    wireframe_config.default_color = Color::srgb(0.2, 0.2, 0.2);

    let texture = textures.get_mut(&atlas_image_handle.0).unwrap();

    texture.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Nearest,
        mipmap_filter: ImageFilterMode::Nearest,
        ..Default::default()
    });

    let atlas_material = atlas_materials.add(BlockAtlasMaterial::new(
        atlas_image_handle.0.clone(),
        &textures,
    ));

    commands.insert_resource(GlobalBlockAtlasMaterial(atlas_material));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1_000.0,
    });

    commands.spawn(PlayerBundle::spawn_at_test_position());
}

#[derive(Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::LoadingAssets), setup)
            .add_plugins(WireframePlugin);
    }
}

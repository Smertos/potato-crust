use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::texture::{
    ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor,
};

use crate::assets::{AppState, BlockTextureAtlasImage};
use crate::block_info::BlockInfoRegistry;
use crate::camera::CameraController;
use crate::material::{BlockAtlasMaterial, BlockAtlasPbrBundle};
use crate::world::chunk::Chunk;

pub fn setup(
    mut commands: Commands,
    atlas_image_handle: Res<BlockTextureAtlasImage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut atlas_materials: ResMut<Assets<BlockAtlasMaterial>>,
    mut textures: ResMut<Assets<Image>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    block_info_registry: Res<BlockInfoRegistry>,
) {
    wireframe_config.global = true;
    wireframe_config.default_color = Color::srgb(0.2, 0.2, 0.2);

    let mut chunk = Chunk::new();
    chunk.update_voxels(&block_info_registry);

    let chunk_mesh = chunk.serialize_voxels_to_render_mesh(&block_info_registry);

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

    commands.spawn(BlockAtlasPbrBundle {
        mesh: meshes.add(chunk_mesh),
        material: atlas_material,
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1_000.0,
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(50.0, 15.0, 50.0))
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..Default::default()
        },
        CameraController::default(),
    ));
}

#[derive(Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_plugins(WireframePlugin);
    }
}

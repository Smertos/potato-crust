pub mod atlas_manager;
pub mod registry;
pub mod textures;

use crate::assets::textures::atlas_texture_info::AtlasTextureInfo;
use bevy::app::AppExit;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use std::path::Path;

use self::atlas_manager::AtlasManager;
use self::registry::{AtlasTextureInfoRegistry, BlockTextureRegistry};
use self::textures::block_texture::BlockTexture;

use crate::block_mesh::BlockMeshStorage;
use crate::material::block_atlas_material::BlockAtlasMaterial;
use crate::material::block_material::BlockMaterial;
use crate::states::GameState;

#[derive(Clone, Resource, TypeUuid)]
#[uuid = "ef87a222-64f6-4add-8a53-108dd0eb9425"]
pub struct UiFont(pub Handle<Font>);

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockTextureRegistry::default())
            .insert_resource(AtlasTextureInfoRegistry::default())
            .insert_resource(BlockMeshStorage::new())
            .insert_resource(AtlasManager::new())
            .add_asset::<AtlasTextureInfo>()
            .add_asset::<BlockAtlasMaterial>()
            .add_asset::<BlockTexture>()
            .add_state::<GameState>()
            .add_systems(
                (load_all_textures, load_block_meshes, load_ui_font)
                    .chain()
                    .in_schedule(OnEnter(GameState::LoadingAssets)),
            )
            .add_system(check_states_loaded.in_set(OnUpdate(GameState::LoadingAssets)))
            .add_system(generate_atlases.in_schedule(OnEnter(GameState::GeneratingAtlases)));
    }
}

pub fn check_states_loaded(
    asset_server: Res<AssetServer>,
    block_textures: Res<Assets<BlockTexture>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let handles = block_textures.iter().map(|(_, x)| x.texture_image.id());

    if let LoadState::Loaded = asset_server.get_group_load_state(handles) {
        next_state.set(GameState::GeneratingAtlases);
        debug!("All assets are loaded!")
    }
}

pub fn generate_atlases(
    mut commands: Commands,
    mut atlas_manager: ResMut<AtlasManager>,
    mut atlas_texture_info_registry: ResMut<AtlasTextureInfoRegistry>,
    mut atlas_texture_infos: ResMut<Assets<AtlasTextureInfo>>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    block_textures: Res<Assets<BlockTexture>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    atlas_manager.process_textures(
        commands,
        &mut atlas_texture_info_registry,
        &mut atlas_texture_infos,
        &mut texture_atlases,
        &mut images,
        &block_textures,
    );

    next_state.set(GameState::InGame);
    debug!("All atlases have been assembled!")
}

pub fn load_all_textures(
    asset_server: Res<AssetServer>,
    mut block_texture_registry: ResMut<BlockTextureRegistry>,
    mut block_materials: ResMut<Assets<BlockMaterial>>,
    mut block_textures: ResMut<Assets<BlockTexture>>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    let io = asset_server.asset_io();
    let textures_path = Path::new("textures");

    let Ok(dirs) = io.read_directory(textures_path) else {
        error!("Failed to read 'assets/textures' directory");
        exit_event_writer.send(AppExit);

        return;
    };

    for x in dirs {
        let Some(file_name) = x.file_name() else {
            continue;
        };

        let Some(file_name) = file_name.to_str() else {
            warn!("File name '{}' is not a valid ITF-8 sequence", file_name.to_string_lossy());
            continue;
        };

        let mut file_name_split = file_name.rsplitn(2, '.');
        let (Some(extension), Some(name)) = (file_name_split.next(), file_name_split.next()) else {
            warn!("File name '{}' is missing png extension or must be an invalid texture", file_name);
            continue;
        };

        // We'll be using png for all our textures
        if extension != "png" {
            continue;
        }

        let asset_path = format!("textures/{}", file_name);
        let image_handle: Handle<Image> = asset_server.load(asset_path);

        // NOTE: we'll later remove material from BlockTexture, since it will be purely used for assembling atlases later on
        let material_handle: Handle<BlockMaterial> =
            block_materials.add(image_handle.clone().into());
        let block_texture_handle = BlockTexture::new(name, material_handle, image_handle);

        let block_texture_handle = block_textures.add(block_texture_handle);

        let insert_result = block_texture_registry
            .0
            .insert(name.to_string(), block_texture_handle);

        if let Err(err) = insert_result {
            error!("Error while adding asset to registry: {}", err);
        }
    }
}

pub fn load_block_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut block_mesh_storage: ResMut<BlockMeshStorage>,
) {
    block_mesh_storage.generate_meshes(&mut meshes);
}

pub fn load_ui_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_font_handle = asset_server.load("fonts/Inter-Regular.ttf");
    let ui_font = UiFont(ui_font_handle);

    commands.insert_resource(ui_font);
}

mod atlas_manager;

use crate::assets::atlas_manager::AtlasManager;
use bevy::app::AppExit;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use std::path::Path;

use crate::block_mesh::BlockMeshStorage;
use crate::block_texture::BlockTexture;
use crate::material::block_material::BlockMaterial;
use crate::registry::BlockTextureRegistry;
use crate::states::GameState;

#[derive(Clone, Resource, TypeUuid)]
#[uuid = "ef87a222-64f6-4add-8a53-108dd0eb9425"]
pub struct UiFont(pub Handle<Font>);

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockTextureRegistry::default())
            .insert_resource(BlockMeshStorage::new())
            .insert_resource(AtlasManager::new())
            .add_asset::<BlockTexture>()
            .add_state(GameState::LoadingAssets)
            .add_system_set(
                SystemSet::on_enter(GameState::LoadingAssets)
                    .with_system(load_all_textures)
                    .with_system(load_block_meshes)
                    .with_system(load_ui_font),
            )
            .add_system_set(
                SystemSet::on_update(GameState::LoadingAssets).with_system(check_states_loaded),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::GeneratingAtlases).with_system(generate_atlases),
            );
    }
}

pub fn check_states_loaded(
    asset_server: Res<AssetServer>,
    block_textures: Res<Assets<BlockTexture>>,
    mut state: ResMut<State<GameState>>,
) {
    let handles = block_textures.iter().map(|(_, x)| x.texture_image.id());

    if let LoadState::Loaded = asset_server.get_group_load_state(handles) {
        if let Err(err) = state.set(GameState::GeneratingAtlases) {
            error!("Game state set error: {}", err);
        } else {
            debug!("All assets are loaded!")
        }
    }
}

pub fn generate_atlases(
    mut atlas_manager: ResMut<AtlasManager>,
    mut images: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    block_textures: Res<Assets<BlockTexture>>,
    mut state: ResMut<State<GameState>>,
) {
    atlas_manager.process_textures(&mut texture_atlases, &mut images, &block_textures);

    // TODO: Figure out better way to go to next state
    if let Err(err) = state.overwrite_set(GameState::InGame) {
        error!("Game state set error: {}", err);
    } else {
        debug!("All atlases have been assembled!")
    }
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

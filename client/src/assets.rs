use bevy::asset::LoadState;
use bevy::prelude::*;
use crate::block_info::BlockInfoPlugin;

pub const ATLAS_TEXTURE_DIRT: u32 = 0;
pub const ATLAS_TEXTURE_GRASS_SIDE: u32 = 1;
pub const ATLAS_TEXTURE_GRASS_TOP: u32 = 2;
pub const ATLAS_TEXTURE_COBBLESTONE: u32 = 3;

#[derive(Default, Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    InGame,
}

#[derive(Resource)]
pub struct BlockTextureAtlasImage(pub Handle<Image>);

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Loaded assets");

    let handle = asset_server.load("textures/experiment-block-atlas.png");
    // let handle = asset_server.load("textures/grass-side.png");
    commands.insert_resource(BlockTextureAtlasImage(handle));
}

pub fn check_loaded_assets(
    mut next_state: ResMut<NextState<AppState>>,
    atlas_image_handle: Res<BlockTextureAtlasImage>,
    asset_server: Res<AssetServer>,
) {
    debug!("Checking if all assets are loaded");
    if let Some(LoadState::Loaded) = asset_server.get_load_state(&atlas_image_handle.0) {
        debug!("All assets are loaded!");
        next_state.set(AppState::InGame);
    }
}

#[derive(Default)]
pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins(BlockInfoPlugin)
            .add_systems(OnEnter(AppState::LoadingAssets), load_assets)
            .add_systems(
                Update,
                check_loaded_assets.run_if(in_state(AppState::LoadingAssets)),
            );
    }
}

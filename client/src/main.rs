use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;

use crate::assets::GameAssetsPlugin;
use crate::camera::CameraControllerPlugin;
use crate::material::BlockAtlasMaterialPlugin;
use crate::setup::SetupPlugin;
use crate::world::WorldPlugin;

mod assets;
mod setup;
mod material;
mod camera;
mod block_info;
mod world;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    font: default(),
                },
            },
        })
        .add_plugins(CameraControllerPlugin)
        .add_plugins(GameAssetsPlugin)
        .add_plugins(BlockAtlasMaterialPlugin)
        .add_plugins(SetupPlugin)
        .add_plugins(WorldPlugin)
        .run();
}

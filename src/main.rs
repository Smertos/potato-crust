use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use input::mouse_grab_input;
use tracing::Level;

mod assets;
mod block_material;
mod block_mesh;
mod block_texture;
mod camera;
mod debug_texture;
mod input;
mod registry;

use crate::assets::{GameAssetsLabel, GameAssetsPlugin};
use crate::block_material::{BlockBundle, BlockMaterial};
use crate::block_mesh::{BlockMeshStorage, BlockSide};
use crate::block_texture::BlockTexture;
use crate::camera::GameplayCameraPlugin;
use crate::debug_texture::uv_debug_texture;
use crate::input::keyboard_input;
use crate::registry::BlockTextureRegistry;

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut block_materials: ResMut<Assets<BlockMaterial>>,
    block_textures: Res<Assets<BlockTexture>>,
    block_texture_registry: Res<BlockTextureRegistry>,
    block_mesh_storage: Res<BlockMeshStorage>,
) {
    let debug_texture = images.add(uv_debug_texture());
    let debug_material = block_materials.add(debug_texture.into());

    let block_material = block_texture_registry
        .get("dirt")
        .and_then(|block_texture| block_textures.get(&block_texture))
        .map(|block_texture| block_texture.material.clone())
        .unwrap_or_else(|| debug_material.clone());

    dbg!(&block_mesh_storage);

    const LIMIT: i32 = 16;

    for y in 0..LIMIT {
        for z in 0..LIMIT {
            for x in 0..LIMIT {
                // TODO: pick mesh depending on surrounding blocks (need to store blocks in level/regions/chunks)
                let has_front_side = z == 0;
                let has_back_side = z == LIMIT - 1;
                let has_left_side = x == 0;
                let has_right_side = x == LIMIT - 1;
                let has_top_side = y == LIMIT - 1;
                let has_bottom_side = y == 0;

                // dbg!(x, y, z, has_front_side, has_back_side, has_left_side, has_right_side, has_top_side, has_bottom_side);

                let mut sides = BlockSide::none();

                if has_front_side {
                    sides |= BlockSide::Front;
                }
                if has_back_side {
                    sides |= BlockSide::Back;
                }
                if has_left_side {
                    sides |= BlockSide::Left;
                }
                if has_right_side {
                    sides |= BlockSide::Right;
                }
                if has_top_side {
                    sides |= BlockSide::Top;
                }
                if has_bottom_side {
                    sides |= BlockSide::Bottom;
                }

                if let Some(cube) = block_mesh_storage.get_mesh(sides) {
                    // TODO: block needs some abstraction (like, we shouldn't be constructing it here like this)
                    commands.spawn(BlockBundle {
                        mesh: cube.clone(),
                        material: block_material.clone(),
                        transform: Transform::from_xyz(
                            0.0 + x as f32,
                            0.0 + y as f32,
                            0.0 + z as f32,
                        ),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,naga=warn,potato_crust=trace".into(),
                    level: Level::DEBUG,
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(MaterialPlugin::<BlockMaterial>::default())
        .add_plugin(GameAssetsPlugin)
        .add_plugin(GameplayCameraPlugin::default())
        .add_startup_system(setup.after(GameAssetsLabel::Loading))
        .add_system(mouse_grab_input)
        .add_system(keyboard_input)
        // .add_system(keyscan_input)
        .run();
}

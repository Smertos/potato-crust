use bevy::log::LogPlugin;
use bevy::prelude::shape::Quad;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::window::PresentMode;
use rand::prelude::SliceRandom;
// use bevy_editor_pls::EditorPlugin;
use input::mouse_grab_input;
use tracing::Level;

mod assets;
mod block_instancing;
mod block_mesh;
mod camera;
mod debug_texture;
mod fps;
mod input;
mod material;
mod states;
mod world;

use crate::assets::registry::AtlasTextureInfoRegistry;
use crate::assets::textures::atlas_texture_info::AtlasTextureInfo;
use crate::assets::GameAssetsPlugin;
use crate::block_instancing::{
    BlockQuadInstanceData, BlockQuadInstanceMaterialData, BlockQuadMaterialPlugin,
};
use crate::block_mesh::{BlockMeshStorage, BlockSide};
use crate::camera::GameplayCameraPlugin;
use crate::debug_texture::uv_debug_texture;
use crate::fps::GameFpsCounterPlugin;
use crate::input::keyboard_input;
use crate::material::block_atlas_material::BlockAtlasMaterial;
use crate::material::block_material::BlockMaterial;
use crate::states::GameState;
use crate::world::block::{BlockBundle, BlockPosition, BlockQuadMesh};

fn get_atlas_block_info(
    name: &'static str,
    atlas_texture_infos: &Res<Assets<AtlasTextureInfo>>,
    texture_info_registry: &Res<AtlasTextureInfoRegistry>,
) -> AtlasTextureInfo {
    texture_info_registry
        .get(name)
        .and_then(|atlas_texture_info_handle| atlas_texture_infos.get(&atlas_texture_info_handle))
        .cloned()
        .expect("to find texture in registry")
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut block_materials: ResMut<Assets<BlockMaterial>>,
    mut block_atlas_materials: ResMut<Assets<BlockAtlasMaterial>>,
    block_atlas_material: Res<BlockAtlasMaterial>,
    block_mesh_storage: Res<BlockMeshStorage>,
) {
    // let debug_texture = images.add(uv_debug_texture());
    // let debug_material = block_materials.add(debug_texture.into());

    let material_handle = block_atlas_materials.add(block_atlas_material.clone());

    let quad_mesh = meshes.add(Quad::default().into());

    const LIMIT: i32 = 16;

    commands.spawn((
        quad_mesh,
        material_handle,
        SpatialBundle::INHERITED_IDENTITY,
        BlockQuadInstanceMaterialData(vec![]),
        BlockQuadMesh,
        NoFrustumCulling,
        // BlockQuadInstanceMaterialData(
        //     (0..LIMIT)
        //         .flat_map(|y| (0..LIMIT).flat_map(move |z| (0..LIMIT).map(move |x| (x, y, z))))
        //         .flat_map(|(x, y, z)| (0..6).map(|side: u32| BlockQuadInstanceData {
        //             position: Vec3::new(x as f32, y as f32, z as f32),
        //             block_side: side,
        //             atlas_section_from: Vec2::new(0.0, 0.0),
        //             atlas_section_to: Vec2::new(1.0, 1.0),
        //         }),
        // ),
    ));

    (0..LIMIT)
        .flat_map(|y| (0..LIMIT).flat_map(move |z| (0..LIMIT).map(move |x| (x, y, z))))
        .for_each(|(x, y, z)| {
            let position = Vec3::new(x as f32, y as f32, z as f32);

            commands.spawn(BlockBundle::new(position));
        })
}

fn collect_block_quads(
    atlas_texture_infos: Res<Assets<AtlasTextureInfo>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_info_registry: Res<AtlasTextureInfoRegistry>,
    block_positions: Query<&BlockPosition>,
    mut block_quad_instance_data: Query<(&mut BlockQuadInstanceMaterialData, With<BlockQuadMesh>)>,
) {
    let names = ["grass-side", "grass-top", "dirt"];
    let mut rng = rand::thread_rng();

    let block_quads: Vec<BlockQuadInstanceData> = block_positions
        .iter()
        .flat_map(|block_position| {
            let block_texture_info = get_atlas_block_info(
                names.choose(&mut rng).unwrap(), // names array is not empty, so it's safe to unwrap
                &atlas_texture_infos,
                &texture_info_registry,
            );
            let (section_from, section_to) = block_texture_info.get_rect(&texture_atlases);

            let quads: Vec<BlockQuadInstanceData> = (0..6)
                .map(move |block_side: u32| BlockQuadInstanceData {
                    position: block_position.0,
                    block_side,
                    atlas_section_from: section_from,
                    atlas_section_to: section_to,
                })
                .collect(); // TODO: del after removing dbg! call

            if block_position.0 == Vec3::ZERO {
                dbg!(&quads);
            }

            quads
        })
        .collect();

    let mut block_quad_instance_data = block_quad_instance_data
        .get_single_mut()
        .expect("could not get block quad instance data");

    block_quad_instance_data.0 .0 = block_quads;
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Potato Crust".into(),
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,naga=warn,client=trace".into(),
                    level: Level::DEBUG,
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(MaterialPlugin::<BlockMaterial>::default())
        .add_plugin(GameAssetsPlugin)
        .add_plugin(GameFpsCounterPlugin)
        .add_plugin(GameplayCameraPlugin::default())
        .add_plugin(BlockQuadMaterialPlugin)
        // .add_plugin(EditorPlugin)
        .add_systems(
            (setup, apply_system_buffers, collect_block_quads)
                .chain()
                .in_schedule(OnEnter(GameState::InGame)),
        )
        .add_systems(
            (mouse_grab_input, keyboard_input)
                .chain()
                .in_set(OnUpdate(GameState::InGame)),
        )
        .run();
}

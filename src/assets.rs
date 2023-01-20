use bevy::prelude::*;

use crate::block_material::BlockMaterial;
use crate::block_mesh::BlockMeshStorage;
use crate::block_texture::BlockTexture;
use crate::registry::BlockTextureRegistry;

#[derive(Clone, Debug, PartialEq, SystemLabel)]
pub enum GameAssetsLabel {
    Loading,
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockTextureRegistry::default())
            .insert_resource(BlockMeshStorage::new())
            .add_asset::<BlockTexture>()
            .add_startup_system(load_block_textures.label(GameAssetsLabel::Loading))
            .add_startup_system(load_block_meshes.label(GameAssetsLabel::Loading));
    }
}

pub fn load_block_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut block_mesh_storage: ResMut<BlockMeshStorage>,
) {
    block_mesh_storage.generate_meshes(&mut meshes);
}

pub fn load_block_textures(
    server: Res<AssetServer>,
    mut block_texture_registry: ResMut<BlockTextureRegistry>,
    mut block_materials: ResMut<Assets<BlockMaterial>>,
    mut block_textures: ResMut<Assets<BlockTexture>>,
) {
    let names = vec!["dirt"];

    for name in names {
        let image_handle: Handle<Image> = server.load("textures/dirt.png");
        let material_handle: Handle<BlockMaterial> =
            block_materials.add(image_handle.clone().into());
        let block_texture = BlockTexture::new("dirt", material_handle, image_handle);

        let block_texture_handle = block_textures.add(block_texture);
        let insert_result = block_texture_registry
            .0
            .insert(name.to_string(), block_texture_handle);

        if let Err(err) = insert_result {
            error!("Error while adding asset to registry: {}", err);
        }
    }
}

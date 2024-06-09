use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, VertexFormat,
};

pub const ATTRIBUTE_ATLAS_TEXTURE_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("AtlasTextureIndex", 6235423423, VertexFormat::Uint32);

#[derive(Asset, AsBindGroup, Clone, Debug, TypePath)]
pub struct BlockAtlasMaterial {
    #[uniform(100)]
    pub texture_size: Vec2,
    #[texture(101)]
    #[sampler(102)]
    pub atlas_texture: Handle<Image>,
}

impl BlockAtlasMaterial {
    pub fn new(atlas_texture: Handle<Image>, textures: &Assets<Image>) -> Self {
        let texture_image = textures
            .get(&atlas_texture)
            .expect("block atlas texture not found");

        Self {
            texture_size: Vec2::new(texture_image.width() as f32, texture_image.height() as f32),
            atlas_texture,
        }
    }
}

impl Material for BlockAtlasMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/block-atlas.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/block-atlas.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            ATTRIBUTE_ATLAS_TEXTURE_INDEX.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

pub struct BlockAtlasMaterialPlugin;

impl Plugin for BlockAtlasMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<BlockAtlasMaterial>::default());
    }
}

pub type BlockAtlasPbrBundle = MaterialMeshBundle<BlockAtlasMaterial>;

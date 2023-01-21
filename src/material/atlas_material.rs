use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[allow(dead_code)]
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "de62a8d7-de20-4c39-9e98-a319f9cff438"]
pub struct AtlasMaterial {
    #[uniform(0)]
    color: Color,
    atlas_offset: Vec2,
    texture_size: Vec2,
    #[texture(3)]
    #[sampler(4)]
    atlas_texture: Handle<Image>,
}

#[allow(dead_code)]
pub type BlockBundle = MaterialMeshBundle<AtlasMaterial>;

impl Material for AtlasMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/atlas-fragment.wgsl".into()
    }
}

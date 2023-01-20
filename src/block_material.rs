use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "e0d30bdd-5f30-4a7d-86f3-b443fcfb4e53"]
pub struct BlockMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
}

pub type BlockBundle = MaterialMeshBundle<BlockMaterial>;

impl Material for BlockMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/block-fragment.wgsl".into()
    }

    // fn vertex_shader() -> ShaderRef {
    //     "shaders/block-vertex.wgsl".into()
    // }
}

impl From<Handle<Image>> for BlockMaterial {
    fn from(color_texture: Handle<Image>) -> Self {
        BlockMaterial {
            color: Color::WHITE,
            color_texture,
        }
    }
}

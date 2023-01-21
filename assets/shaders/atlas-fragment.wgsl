struct BlockMaterial {
    color: vec4<f32>,
    atlas_offset: vec2<f32>,
    texture_size: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> material: BlockMaterial;

@group(1) @binding(3)
var atlas_texture: texture_2d<f32>;

@group(1) @binding(4)
var atlas_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
//    let fixed_uv = vec2<f32>(uv.x, 1.0 - uv.y);
    return material.color * vec4<f32>(1.0, 1.0, 1.0, 1.0) *  textureSample(atlas_texture, atlas_sampler, uv);
}
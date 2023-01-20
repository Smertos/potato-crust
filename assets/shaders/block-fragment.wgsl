struct BlockMaterial {
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: BlockMaterial;

@group(1) @binding(1)
var color_texture: texture_2d<f32>;

@group(1) @binding(2)
var color_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
//    let fixed_uv = vec2<f32>(uv.x, 1.0 - uv.y);
    return material.color * vec4<f32>(1.0, 1.0, 1.0, 1.0) *  textureSample(color_texture, color_sampler, uv);
}
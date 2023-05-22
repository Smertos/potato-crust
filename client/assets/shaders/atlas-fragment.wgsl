struct BlockMaterial {
    sections: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: BlockMaterial;

@group(1) @binding(1)
var atlas_texture: texture_2d<f32>;

@group(1) @binding(2)
var atlas_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let section_from: vec2<f32> = material.sections.xy;
    let section_to: vec2<f32> = material.sections.zw;

    let atlas_texture_size_i: vec2<i32> = textureDimensions(atlas_texture);
    let atlas_texture_size: vec2<f32> = vec2<f32>(f32(atlas_texture_size_i.x), f32(atlas_texture_size_i.y));
    let section_size = section_to - section_from;

    let section_perc = (section_size.xy * uv.xy);
    let vert_uv_coord = (section_from + section_perc);
    let section_uv = (vert_uv_coord / atlas_texture_size.xy);

    /* return vec4<f32>(1.0, 1.0, 1.0, 1.0) *  textureSample(atlas_texture, atlas_sampler, section_uv); */
    return vec4<f32>(1.0, 1.0, 1.0, 1.0) *  textureSample(atlas_texture, atlas_sampler, section_uv);
}

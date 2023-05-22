
@vertex
fn vertex(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
//    let fixed_uv = vec2<f32>(uv.x, 1.0 - uv.y);
    return material.color * vec4<f32>(1.0, 1.0, 1.0, 1.0) *  textureSample(atlas_texture, atlas_sampler, uv);
}
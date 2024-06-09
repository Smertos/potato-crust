#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

const ATLAS_TEXTURES_PER_SIDE: f32 = 16.0;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) atlas_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) atlas_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.tex_coords = vertex.tex_coords;
    out.atlas_index = vertex.atlas_index;
    return out;
}

struct CustomMaterial {
    texture_size: vec2<f32>,
};

@group(2) @binding(100) var<uniform> material: CustomMaterial;
@group(2) @binding(101) var atlas_texture: texture_2d<f32>;
@group(2) @binding(102) var atlas_texture_sampler: sampler;

struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
    @location(1) atlas_index: u32,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    let atlas_index: f32 = f32(input.atlas_index); // 3
    let texture_size: vec2<f32> = material.texture_size; // 256, 256
    let atlas_textures_per_side: vec2<f32> = vec2<f32>(ATLAS_TEXTURES_PER_SIDE, ATLAS_TEXTURES_PER_SIDE); // 8, 8
    let section_size: vec2<f32> = texture_size / atlas_textures_per_side; // 16, 16

    let atlas_x: f32 = atlas_index % ATLAS_TEXTURES_PER_SIDE; // 1
    let atlas_y: f32 = floor(atlas_index / ATLAS_TEXTURES_PER_SIDE); // 0

    let section_from: vec2<f32> = vec2<f32>(atlas_x, atlas_y) * section_size; // 16, 0
    let section_uv_offset: vec2<f32> = (section_size * input.tex_coords % section_size); // 16, 0 * 0.75, 0.5 = 12, 0

    let section_uv: vec2<f32> = (section_from + section_uv_offset); // 28, 0

//    return textureSample(atlas_texture, atlas_texture_sampler, input.tex_coords);
    return textureSample(atlas_texture, atlas_texture_sampler, section_uv / texture_size);
}
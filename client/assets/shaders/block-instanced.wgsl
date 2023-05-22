#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

// NOTE binding must come before functions
#import bevy_pbr::mesh_functions

/*
 * Block Index
 * 0 = front
 * 1 = back
 * 2 = left
 * 3 = right
 * 4 = top
 * 5 = bottom
 */

//fn transform_position(position: vec3<f32>, block_side: u32) -> vec3<f32> {
//    let block_side = f32(block_side);
//    let block_side = block_side * 2.0 - 1.0;
//    let block_side = vec3<f32>(block_side, block_side, block_side);
//    return position * block_side;
//}

 fn quad_to_cube_position(coords: vec2<f32>, side_index: u32) -> vec3<f32> {
   var position: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

   if (side_index == 0u) {
     // Front side
     position = vec3<f32>(coords - vec2<f32>(0.5, 0.5), 0.5);
   } else if (side_index == 1u) {
     // Back side
     position = vec3<f32>(vec2<f32>(1.0, 1.0) - coords - vec2<f32>(0.5, 0.5), -0.5);
   } else if (side_index == 2u) {
     // Left side
     position = vec3<f32>(-0.5, coords.y - 0.5, 1.0 - coords.x - 0.5);
   } else if (side_index == 3u) {
     // Right side
     position = vec3<f32>(0.5, coords.y - 0.5, coords.x - 0.5);
   } else if (side_index == 4u) {
     // Top side
     position = vec3<f32>(coords - vec2<f32>(0.5, 0.5), -0.5);
   } else if (side_index == 5u) {
     // Bottom side
     position = vec3<f32>(coords - vec2<f32>(0.5, 0.5), 0.5);
   }

   return position;
 }

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) position: vec3<f32>,
    @location(4) block_side: u32,
    @location(5) atlas_sections: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) atlas_sections: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let local_position = quad_to_cube_position(vertex.position.xy, vertex.block_side) + vertex.position;

    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.uv = vertex.uv;
    out.atlas_sections = vertex.atlas_sections;
    return out;
}

@group(2) @binding(0)
var atlas_texture: texture_2d<f32>;

@group(2) @binding(1)
var atlas_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let section_from: vec2<f32> = in.atlas_sections.xy;
    let section_to: vec2<f32> = in.atlas_sections.zw;

    let atlas_texture_size_i: vec2<i32> = textureDimensions(atlas_texture);
    let atlas_texture_size: vec2<f32> = vec2<f32>(f32(atlas_texture_size_i.x), f32(atlas_texture_size_i.y));
    let section_size = section_to - section_from;

    let section_perc = (section_size.xy * in.uv);
    let vert_uv_coord = (section_from + section_perc);
    let section_uv = (vert_uv_coord / atlas_texture_size.xy);

    return vec4<f32>(1.0, 1.0, 1.0, 1.0) *  textureSample(atlas_texture, atlas_sampler, section_uv);
}

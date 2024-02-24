#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world}
#import "shaders/config.wgsl"::{fern_vertices, Vertex}
#import bevy_pbr::{prepass_io::{VertexOutput}}

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

struct CustomMaterial {
    time: f32,
};

@group(1) @binding(100) var<uniform> material: CustomMaterial;


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let res = fern_vertices(globals.time, vertex);
    let model = get_model_matrix(vertex.instance_index);
    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(model, vec4<f32>(res.pos, 1.0));
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(res.pos, 1.0));
    out.clip_position_unclamped = out.position;
    out.position.z = min(out.position.z, 1.0);
    out.instance_index = vertex.instance_index;
    out.uv = res.uv;
    return out;
}

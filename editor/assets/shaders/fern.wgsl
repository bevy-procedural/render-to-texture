#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_tangent_local_to_world, mesh_position_local_to_world,mesh_position_local_to_clip}
#import "shaders/config.wgsl"::{fern_vertices, Vertex}
#import bevy_pbr::{forward_io::VertexOutput}
// TODO: until 13.1: mesh_view_bindings::globals, 

struct CustomMaterial {
    time: f32,
};

@group(1) @binding(100) var<uniform> material: CustomMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    // TODO: until 13.1 globals.time
    let res = fern_vertices(1.0, vertex);
    let model = get_model_matrix(vertex.instance_index);
    var out = VertexOutput();
    out.position = mesh_position_local_to_clip(model, vec4<f32>(res.pos, 1.0));
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(res.pos, 1.0));
    out.world_normal = (model * vec4<f32>(res.normal, 0.0)).xyz;
    out.color = vec4<f32>(res.ao, res.ao, res.ao, 1.0);
    out.instance_index = vertex.instance_index;
    //out.world_tangent = mesh_tangent_local_to_world(model, res.tangent, vertex.instance_index);
    out.uv = res.uv;
    return out;
}

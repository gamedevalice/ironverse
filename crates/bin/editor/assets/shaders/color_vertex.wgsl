#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::mesh_functions mesh_position_local_to_world
#import bevy_pbr::mesh_functions mesh_position_local_to_clip



struct Vertex {
  @location(0) position: vec3<f32>,
  @location(1) normal: vec3<f32>,
  @location(2) color: vec3<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) color: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
  var out: VertexOutput;
  out.world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
  out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
  out.world_normal = vertex.normal;

  out.color = vertex.color;
  return out;
}

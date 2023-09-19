#import bevy_pbr::pbr_functions pbr
#import bevy_pbr::pbr_functions pbr_input_new
#import bevy_pbr::pbr_functions PbrInput
#import bevy_pbr::pbr_functions prepare_world_normal
#import bevy_pbr::pbr_functions calculate_view
#import bevy_core_pipeline::tonemapping tone_mapping
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings view

struct CustomMaterial {
  base_color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,

  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) color: vec3<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
  var pbr_input: PbrInput = pbr_input_new();
  pbr_input.material.base_color = vec4<f32>(input.color, 1.0);
  pbr_input.frag_coord = input.frag_coord;
  pbr_input.world_position = input.world_position;

  pbr_input.world_normal = prepare_world_normal(
    input.world_normal,
    true,
    false,
  );

  pbr_input.N = normalize(input.world_normal);
  pbr_input.V = calculate_view(input.world_position, pbr_input.is_orthographic);

  return tone_mapping(pbr(pbr_input), view.color_grading);
  //return vec4<f32>(input.color, 1.0);
}





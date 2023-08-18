#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions
#import bevy_pbr::pbr_ambient

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

  return tone_mapping(pbr(pbr_input));
  // return vec4<f32>(input.color, 1.0);
}





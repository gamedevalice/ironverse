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
  // @builtin(position) frag_coord: vec4<f32>,
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,

  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) color: vec3<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
  // let pos = seamless_pos(input.world_position.xyz);
  // var color = triplanar_color(pos, input) * material.base_color;
  // color = normalize(color);

  // var pbr_input: PbrInput = pbr_input_new();
  // pbr_input.material.base_color = pbr_input.material.base_color * color;
  // pbr_input.frag_coord = input.frag_coord;
  // pbr_input.world_position = input.world_position;
  // pbr_input.world_normal = prepare_world_normal(
  //   input.world_normal,
  //   true,
  //   false,
  // );

  // pbr_input.is_orthographic = view.projection[3].w == 1.0;

  // let sharpness_1 = 8.0;
  // var weights_1 = pow(abs(input.world_normal), vec3(sharpness_1));
  // weights_1 = weights_1 / (weights_1.x + weights_1.y + weights_1.z);

  // let scale = 1.0;
  // let uv_x = pos.zy * scale;
  // let uv_y = pos.xz * scale;
  // let uv_z = pos.xy * scale;
  // var triplanar = Triplanar(weights_1, uv_x, uv_y, uv_z);

  // pbr_input.N = triplanar_normal_to_world_splatted(
  //   material.flags,
  //   input.voxel_weight, 
  //   input.world_normal, 
  //   input.voxel_type_1, 
  //   triplanar
  // );

  // pbr_input.V = calculate_view(input.world_position, pbr_input.is_orthographic);

  // return tone_mapping(pbr(pbr_input));



  return vec4<f32>(input.color, 1.0);
  // return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}





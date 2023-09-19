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
  flags: u32,
  uv_scale: f32,
}

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var albedo: texture_2d_array<f32>;
@group(1) @binding(2)
var albedo_sampler: sampler;
@group(1) @binding(3)
var normal_texture: texture_2d_array<f32>;
@group(1) @binding(4)
var normal_sampler: sampler;

struct FragmentInput {
  // @builtin(position) frag_coord: vec4<f32>,
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,

  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) voxel_weight: vec4<f32>,
  @location(3) voxel_type_1: vec4<u32>,

  // #import bevy_pbr::mesh_vertex_output
};


struct Triplanar {
  w: vec3<f32>,
  uv_x: vec2<f32>,
  uv_y: vec2<f32>,
  uv_z: vec2<f32>,
}

fn sample_normal_map(
  flags: u32,
  uv: vec2<f32>, 
  material_type: i32
  ) -> vec3<f32> {
  var Nt = textureSample(normal_texture, normal_sampler, uv, material_type).rgb;
  if ((flags & STANDARD_MATERIAL_FLAGS_TWO_COMPONENT_NORMAL_MAP) != 0u) {
    Nt = vec3<f32>(Nt.rg * 2.0 - 1.0, 0.0);
    Nt.z = sqrt(1.0 - Nt.x * Nt.x - Nt.y * Nt.y);
  } else {
    Nt = Nt * 2.0 - 1.0;
  }
  if ((flags & STANDARD_MATERIAL_FLAGS_FLIP_NORMAL_MAP_Y) != 0u) {
    Nt.y = -Nt.y;
  }
  return normalize(Nt);
}

fn triplanar_normal_to_world(
  flags: u32,
  material_type: i32, 
  world_normal: vec3<f32>, 
  triplanar: Triplanar,
) -> vec3<f32> {

  var normal_x = sample_normal_map(flags, triplanar.uv_x, material_type);
  var normal_y = sample_normal_map(flags, triplanar.uv_y, material_type);
  var normal_z = sample_normal_map(flags, triplanar.uv_z, material_type);

  normal_x = vec3(normal_x.xy + world_normal.yz, abs(normal_x.z) * world_normal.x);
  normal_y = vec3(normal_y.xy + world_normal.zx, abs(normal_y.z) * world_normal.y);
  normal_z = vec3(normal_z.xy + world_normal.xy, abs(normal_z.z) * world_normal.z);

  return normalize(
    normal_x.zxy * triplanar.w.x +
    normal_y.yzx * triplanar.w.y +
    normal_z.xyz * triplanar.w.z
  );
  
  // return vec3<f32>(0.0);
}

fn triplanar_normal_to_world_splatted(
  flags: u32,
  material_weights: vec4<f32>,
  world_normal: vec3<f32>,
  material_types: vec4<u32>,
  triplanar: Triplanar,
) -> vec3<f32> {

  var sum = vec3(0.0);
  if material_weights.x > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(flags, i32(material_types.x), world_normal, triplanar);
  }
  if material_weights.y > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(flags, i32(material_types.y), world_normal, triplanar);
  }
  if material_weights.z > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(flags, i32(material_types.z), world_normal, triplanar);
  }
  if material_weights.w > 0.0 {
    sum += material_weights.x * triplanar_normal_to_world(flags, i32(material_types.w), world_normal, triplanar);
  }

  // return vec3<f32>(0.0);
  return normalize(sum);
}

fn seamless_pos(world_pos: vec3<f32>) -> vec3<f32> {
  var pos = world_pos % 1.0;
  if pos.x < 0.0 {
    pos.x += 1.0;
  }
  if pos.y < 0.0 {
    pos.y += 1.0;
  }
  if pos.z < 0.0 {
    pos.z += 1.0;
  }
  return pos;
}

fn triplanar_color(world_pos: vec3<f32>, input: FragmentInput) -> vec4<f32> {
  let zy = world_pos.zy;
  let xz = world_pos.xz;
  let xy = world_pos.xy;

  var dx0 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.x));
  var dy0 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.x));
  var dz0 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.x));

  var dx1 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.y));
  var dy1 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.y));
  var dz1 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.y));

  var dx2 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.z));
  var dy2 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.z));
  var dz2 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.z));

  var dx3 = textureSample(albedo, albedo_sampler, zy, i32(input.voxel_type_1.w));
  var dy3 = textureSample(albedo, albedo_sampler, xz, i32(input.voxel_type_1.w));
  var dz3 = textureSample(albedo, albedo_sampler, xy, i32(input.voxel_type_1.w));

  let filter_0 = 100;
  let filter_1 = 100;
  let filter_2 = 100;

  var index0 = 1.0;
  if i32(input.voxel_type_1.x) == filter_0 || i32(input.voxel_type_1.x) == filter_1
  || i32(input.voxel_type_1.x) == filter_2 {
    index0 = 0.0;
  }
  var index1 = 1.0;
  if i32(input.voxel_type_1.y) == filter_0 || i32(input.voxel_type_1.y) == filter_1
  || i32(input.voxel_type_1.y) == filter_2 {
    index1 = 0.0;
  }
  var index2 = 1.0;
  if i32(input.voxel_type_1.z) == filter_0 || i32(input.voxel_type_1.z) == filter_1
  || i32(input.voxel_type_1.z) == filter_2 {
    index2 = 0.0;
  }
  var index3 = 1.0;
  if i32(input.voxel_type_1.w) == filter_0 || i32(input.voxel_type_1.w) == filter_1
  || i32(input.voxel_type_1.w) == filter_2 {
    index3 = 0.0;
  }

  var vx = input.voxel_weight.x * index0;
  var vy = input.voxel_weight.y * index1;
  var vz = input.voxel_weight.z * index2;
  var vw = input.voxel_weight.w * index3;

  // Normalize the result

  var dx = dx0 * vx + dx1 * vy +
    dx2 * vz + dx3 * vw;

  var dy = dy0 * vx + dy1 * vy +
    dy2 * vz + dy3 * vw;

  var dz = dz0 * vx + dz1 * vy +
    dz2 * vz + dz3 * vw;

  let dx_normal = dpdx(input.world_position);
  let dy_normal = dpdy(input.world_position);
  // let cross = cross(dx_normal, dy_normal); // Error in WebGPU
  let a = vec3<f32>(dx_normal.x, dx_normal.y, dx_normal.z);
  let b = vec3<f32>(dy_normal.x, dy_normal.y, dy_normal.z);
  let normal = normalize(cross(a, b));

  // let normal = input.world_normal;

  let sharpness = 10.0;
  var weights = pow(abs(normal.xyz), vec3<f32>(sharpness, sharpness, sharpness));
  weights = weights / (weights.x + weights.y + weights.z);

  var color = dx * weights.x + dy * weights.y + dz * weights.z;
  return color;
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
  let pos = seamless_pos(input.world_position.xyz);
  var color = triplanar_color(pos, input) * material.base_color;
  color = normalize(color);

  var pbr_input: PbrInput = pbr_input_new();
  pbr_input.material.base_color = pbr_input.material.base_color * color;
  pbr_input.frag_coord = input.frag_coord;
  pbr_input.world_position = input.world_position;
  pbr_input.world_normal = prepare_world_normal(
    input.world_normal,
    true,
    false,
  );

  pbr_input.is_orthographic = view.projection[3].w == 1.0;

  let sharpness_1 = 8.0;
  var weights_1 = pow(abs(input.world_normal), vec3(sharpness_1));
  weights_1 = weights_1 / (weights_1.x + weights_1.y + weights_1.z);

  let scale = 1.0;
  let uv_x = pos.zy * scale;
  let uv_y = pos.xz * scale;
  let uv_z = pos.xy * scale;
  var triplanar = Triplanar(weights_1, uv_x, uv_y, uv_z);

  pbr_input.N = triplanar_normal_to_world_splatted(
    material.flags,
    input.voxel_weight, 
    input.world_normal, 
    input.voxel_type_1, 
    triplanar
  );

  pbr_input.V = calculate_view(input.world_position, pbr_input.is_orthographic);

  return tone_mapping(pbr(pbr_input));



  // return color;
  // return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}





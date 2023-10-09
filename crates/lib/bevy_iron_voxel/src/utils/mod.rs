#![allow(dead_code, unused_variables)]  // Forced to use, even though the look_at_to_rotation_quat() is being used, it is still showing warning


use bevy::math::{Vec3, Quat};
use bevy::prelude::Transform;
use voxels::chunk::chunk_manager::ChunkManager;
use voxels::chunk::adjacent_keys_i64;
use voxels::chunk::voxel_pos_to_key;

pub struct Math;

impl Math {
  pub fn look_at_to_rotation_quat(look_at: Vec3) -> Quat {
    let rot = Math::look_at_to_rotation(look_at);
    // Quat::from_rotation_ypr(rot.y, rot.x, 0.0)
    Quat::from_rotation_y(rot.y) * Quat::from_rotation_x(rot.x)
  }

  pub fn look_at_to_rotation(look_at: Vec3) -> Vec3 {
    let tmp_look_at = look_at.normalize();
    let mut rad_x = tmp_look_at.y;
    if rad_x.is_nan() {
      rad_x = 0.0;
    }

    let mut rad_y = tmp_look_at.x / tmp_look_at.z;
    if rad_y.is_nan() {
      rad_y = 0.0;
    }

    
    let mut y_rot = rad_y.atan();
    if tmp_look_at.z > 0.0 {
      let half_pi = std::f32::consts::PI * 0.5;
      y_rot = -((half_pi) + (half_pi - y_rot));
    }

    Vec3::new(rad_x.asin(), y_rot, 0.0)
  }

  pub fn rot_to_look_at(rot: Vec3) -> Vec3 {
    let yaw = -rot.y - std::f32::consts::PI * 0.5;

    let len = rot.x.cos();
    return Vec3::new(yaw.cos() * len, rot.x.sin(), yaw.sin() * len).normalize();
  }
}

pub fn to_key(translation: &Vec3, seamless_size: u32) -> [i64; 3] {
  let pos = [translation.x as i64, translation.y as i64, translation.z as i64];
  voxel_pos_to_key(&pos, seamless_size)
}

#[derive(Clone)]
pub struct MeshColliderData {
  // pub positions: Vec<Point<f32>>,
  pub positions: Vec<Vec3>,
  pub indices: Vec<[u32; 3]>,
}

// TODO: Refactor, change name for clarity
pub fn nearest_voxel_point(
  chunk_manager: &ChunkManager,
  intersection: Vec3,
  _include_current: bool,
  voxel: u8,
) -> Option<[i64; 3]> {
  let point = [
    (intersection.x.round()) as i64,
    (intersection.y.round()) as i64,
    (intersection.z.round()) as i64,
  ];

  let mut shortest_dist = f32::MAX;

  let mut nearest = None;
  let points_around = adjacent_keys_i64(&point, 1, true);
  for pa in points_around.iter() {
    let val = chunk_manager.get_voxel(pa);
    if val == voxel {
      let current_point = Vec3::new(pa[0] as f32, pa[1] as f32, pa[2] as f32);

      let dist = intersection - current_point;
      // println!("tmp2 {:?} {:?}", (tmp_point[0], tmp_point[1], tmp_point[2]), dist);
      if shortest_dist > dist.length_squared() {
        shortest_dist = dist.length_squared();
        nearest = Some(pa.clone());
      }
    }
  }
  return nearest;
}

// TODO: Refactor, change name for clarity
pub fn nearest_voxel_point_0(
  chunk_manager: &ChunkManager,
  intersection: Vec3,
  _include_current: bool,
) -> Option<[i64; 3]> {
  let point = [
    (intersection.x.round()) as i64,
    (intersection.y.round()) as i64,
    (intersection.z.round()) as i64,
  ];

  let mut shortest_dist = f32::MAX;
  let mut nearest = None;
  let points_around = adjacent_keys_i64(&point, 1, true);
  for pa in points_around.iter() {
    let val = chunk_manager.get_voxel(pa);
    if val > 0 {
      let current_point = Vec3::new(pa[0] as f32, pa[1] as f32, pa[2] as f32);

      let dist = intersection - current_point;
      // println!("tmp2 {:?} {:?}", (tmp_point[0], tmp_point[1], tmp_point[2]), dist);
      if shortest_dist > dist.length_squared() {
        shortest_dist = dist.length_squared();
        nearest = Some(pa.clone());
      }
    }
  }
  return nearest;
}



pub struct RayUtils;

impl RayUtils {
  pub fn get_normal_point(
    trans: &Transform, dist: f32, size: u32
  ) -> Vec3 {
    let mut point = trans.translation + trans.forward() * dist;
    point -= size as f32 * 0.5 - 0.5;
  
    RayUtils::get_snapped_position(point, 1)
  }

  pub fn get_snapped_point(
    trans: &Transform, dist: f32, size: u32
  ) -> Vec3 {
    let mut point = trans.translation + trans.forward() * dist;
    point -= size as f32 * 0.5 - 0.5;
  
    RayUtils::get_snapped_position(point, size)
  }

  fn get_snapped_position(pos: Vec3, size: u32) -> Vec3 {
    let adj_positions = RayUtils::get_nearby_snapped_positions(pos, size);
  
    let mut min_dist = f32::MAX;
    let mut snapped_pos = Vec3::ZERO;
    for adj_pos in adj_positions.iter() {
      let dist = pos.distance_squared(*adj_pos);
  
      if dist < min_dist {
        min_dist = dist;
        snapped_pos = *adj_pos;
      }
    }
  
    snapped_pos
  }
  
  fn get_nearby_snapped_positions(pos: Vec3, size: u32) -> Vec<Vec3> {
    let mut result = Vec::new();
  
    let size_i64 = size as i64;
    let base_x = ( (pos.x.round() as i64) / size_i64 ) * size_i64;
    let base_y = ( (pos.y.round() as i64) / size_i64 ) * size_i64;
    let base_z = ( (pos.z.round() as i64) / size_i64 ) * size_i64;
  
    // println!("base_x {}", base_x);
  
    let range = 1;
    let min = -range;
    let max = range + 1;
    for x in min..max {
      for y in min..max {
        for z in min..max {
          let adj_x = base_x + (x * size_i64);
          let adj_y = base_y + (y * size_i64);
          let adj_z = base_z + (z * size_i64);
  
          result.push(Vec3::new(adj_x as f32, adj_y as f32, adj_z as f32));
  
          // println!("adj_x {}", adj_x);
        }
      }
    }
    
  
    result
  }


  pub fn get_normal_point_with_scale(
    trans: &Transform, dist: f32, size: u32, scale: f32
  ) -> Vec3 {
    let mut _point = trans.translation + trans.forward() * dist;
    // point -= size as f32 * 0.5 - 0.5;

    /*
      Need to return divisible by scale, Ex: 0.0, 0.5, 1.0
     */

    Vec3::ZERO
  }
  
}

#[cfg(test)]
mod tests {
  use bevy::prelude::Transform;
  use super::RayUtils;


  #[test]
  fn test_potential_positions() -> Result<(), String> {

    RayUtils::get_normal_point_with_scale(
      &Transform::default(), 1.0, 1, 0.5
    );

    Ok(())
  }
}

#![allow(dead_code, unused_variables)]  // Forced to use, even though the look_at_to_rotation_quat() is being used, it is still showing warning

use std::fs::File;

use bevy::math::{Vec3, Quat};
use voxels::chunk::chunk_manager::ChunkManager;
use voxels::chunk::{adjacent_keys_i64};
use voxels::{chunk::{voxel_pos_to_key}};

use crate::data::{Data, Status, Terrains};

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




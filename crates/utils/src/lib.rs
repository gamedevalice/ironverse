use bevy::prelude::{Transform, Vec3};

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
    let mut point = trans.translation + trans.forward() * dist;
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

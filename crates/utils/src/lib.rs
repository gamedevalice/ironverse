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
    trans: &Transform, dist: f32, scale: f32
  ) -> Vec3 {
    let mut point = trans.translation + trans.forward() * dist;
    // point -= size as f32 * 0.5 - 0.5;

    /*
      Need to return divisible by scale, Ex: 0.0, 0.5, 1.0
     */

    Vec3::ZERO
  }

  pub fn get_nearest_coord(pos: Vec3, scale: f32) -> Vec3 {
    let mut nearest = Vec3::NAN;


    nearest
  }
  
}

fn round(num: f32, nearest: f32) -> f32 {
  let div = 1.0 / nearest;

  let half = nearest / 2.0;
  
  let base_div = (num / nearest).floor();
  let base_val = nearest * base_div;

  let modulus = num % nearest;

  
  let mut res = base_val;
  if modulus >= half {
    res += nearest;
  }

  println!(
    "num {}, div {}, half {} base_div {}, base_val {}, modulus {}", 
    num, div, half, base_div, base_val, modulus
  );



  res
}

#[cfg(test)]
mod tests {
  use bevy::prelude::{Transform, Vec3};
  use crate::round;

use super::RayUtils;


  #[test]
  fn test_nearest_positive_positions_by_1_0() -> Result<(), String> {
    // let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
    //   .looking_to(Vec3::Z, Vec3::Y);

    // let dist = 0.0;
    // let scale = 1.0;

    // RayUtils::get_normal_point_with_scale(
    //   &transform, 1.0, 0.5
    // );

    
    /*
      Ex:
        1:
          Position: 0.0, 0.0, 0.0
          Scale: 1.0
          Result: 0.0, 0.0, 0.0
        2:
          Position: 0.4, 0.4, 0.4
          Scale: 1.0
          Result: 0.0, 0.0, 0.0
        3:
          Position: 0.5, 0.5, 0.5
          Scale: 1.0
          Result: 1.0, 1.0, 1.0

      Scale down the problem for now
      Ex:
        1. 
          Position: 0.0
          Scale: 1.0
          Result: 0.0
        2. 
          Position: 0.4
          Scale: 1.0
          Result: 0.0
        3. 
          Position: 0.5
          Scale: 1.0
          Result: 1.0

        3. 
          Position: 0.0
          Scale: 0.5
          Result: 0.0
        4. 
          Position: 0.24
          Scale: 0.5
          Result: 0.0
        5. 
          Position: 0.25
          Scale: 0.5
          Result: 0.5

        6. 
          Position: -0.24
          Scale: 0.5
          Result: 0.0
        7. 
          Position: -0.25
          Scale: 0.5
          Result: -0.5
     */

    
    /*
      Scale = 1.0
        Res = -1.0  if pos > -1.5   && pos <= -0.5
        Res =  0.0  if pos > -0.5   && pos <=  0.5
        Res =  1.0  if pos >  0.5   && pos <=  1.5

      Scale = 0.5
        Res =  0.0  if pos > -0.25  && pos <=  0.25
        Res =  0.5  if pos >  0.25  && pos <=  0.75
        Res =  1.0  if pos >  0.75  && pos <=  1.25

      Scale = 0.25
        Res =  0.0  if pos > -0.125 && pos <=  0.125
        Res =  0.25 if pos >  0.125 && pos <=  0.375
        Res =  0.5  if pos >  0.375 && pos <=  0.625
        Res =  0.75 if pos >  0.625 && pos <=  0.875
        Res =  1.0  if pos >  0.875 && pos <=  1.125
     */
    /*
      Get the base
      Or multiple by the div
      Then divide after

      Ex:
        Scale = 1.0
          Pos = 0.4
          Res = 0.0
        


     */
    
    let scale = 1.0;
    assert_eq!(round(0.3,   scale), 0.0);
    assert_eq!(round(0.49,  scale), 0.0);
    assert_eq!(round(0.5,   scale), 1.0);
    assert_eq!(round(0.99,  scale), 1.0);

    assert_eq!(round(1.3,   scale), 1.0);
    assert_eq!(round(1.49,  scale), 1.0);
    assert_eq!(round(1.5,   scale), 2.0);
    assert_eq!(round(1.99,  scale), 2.0);

    let scale = 0.5;
    assert_eq!(round(0.1,   scale), 0.0);
    assert_eq!(round(0.24,  scale), 0.0);
    assert_eq!(round(0.25,  scale), 0.5);
    assert_eq!(round(0.49,  scale), 0.5);
    
    assert_eq!(round(0.6,   scale), 0.5);
    assert_eq!(round(0.749, scale), 0.5);
    assert_eq!(round(0.75,  scale), 1.0);
    assert_eq!(round(0.99,  scale), 1.0);

    assert_eq!(round(1.1,   scale), 1.0);
    assert_eq!(round(1.24,  scale), 1.0);
    assert_eq!(round(1.25,  scale), 1.5);
    assert_eq!(round(1.49,  scale), 1.5);
    
    assert_eq!(round(1.6,   scale), 1.5);
    assert_eq!(round(1.749, scale), 1.5);
    assert_eq!(round(1.75,  scale), 2.0);
    assert_eq!(round(1.99,  scale), 2.0);


    let scale = 0.25;
    assert_eq!(round(0.1,   scale), 0.0);
    assert_eq!(round(0.124, scale), 0.0);
    assert_eq!(round(0.125, scale), 0.25);
    assert_eq!(round(0.249, scale), 0.25);

    assert_eq!(round(0.26,  scale), 0.25);
    assert_eq!(round(0.374, scale), 0.25);
    assert_eq!(round(0.375, scale), 0.5);
    assert_eq!(round(0.499, scale), 0.5);


    Ok(())
  }
}

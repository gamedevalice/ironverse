pub struct RayUtils;

impl RayUtils {
  pub fn get_normal_point_with_scale(
    pos: [f32; 3], forward: [f32; 3], dist: f32, nearest: f32
  ) -> [f32; 3] {
    let point = [
      pos[0] + forward[0] * dist,
      pos[1] + forward[1] * dist,
      pos[2] + forward[2] * dist,
    ];
    
    [
      round(point[0], nearest),
      round(point[1], nearest),
      round(point[2], nearest),
    ]
  }


  pub fn get_nearest_coord(pos: [f32; 3], nearest: f32) -> [f32; 3] {
    [round(pos[0], nearest), 
      round(pos[1], nearest), 
      round(pos[2], nearest)]
  }
  
}

fn round(num: f32, nearest: f32) -> f32 {
  let half = nearest / 2.0;
  let mut base_div = (num / nearest).floor();
  if num < 0.0 {
    base_div = (num / nearest).ceil();
  }

  let base_val = nearest * base_div;
  let modulus = num % nearest;
  let mut res = base_val;

  if modulus.abs() >= half {
    if num < 0.0 { 
      res -= nearest;
    } else {
      res += nearest
    }
  }
  res
}

pub struct Utils;
impl Utils {
  pub fn get_keys_by_tile_dist(key: &[i64; 3], min: i64, max: i64) -> Vec<[i64; 3]> {
    let mut keys = Vec::new();
    let mut tmp = [0; 3];

    let start = -max;
    let end = max + 1;
    for x in start..end {
      for y in start..end {
        for z in start..end {
          tmp[0] = key[0] + x;
          tmp[1] = key[1] + y;
          tmp[2] = key[2] + z;

          if min == 0 {
            let range = Utils::get_tile_range(key, &tmp);
            if range <= max {
              keys.push(tmp);
              // println!("key {:?}", tmp);
            }
          }

          if min > 0 {
            let range = Utils::get_tile_range(key, &tmp);
            if range >= min && range <= max {
              keys.push(tmp);
              // println!("key {:?}", tmp);
            }
          }

          
        }
      }
    }

    keys
  }

  pub fn get_tile_range(key1: &[i64; 3], key2: &[i64; 3]) -> i64 {
    let mut range = 0;
    for i in 0..key1.len() {
      let r = (key1[i] - key2[i]).abs();
      if r > range {
        range = r;
      }
    }
    range
  }

  pub fn get_keys_by_dist(key: &[i64; 3], min: i64, max: i64) -> Vec<[i64; 3]> {
    let mut keys = Vec::new();
    let mut tmp = [0; 3];
    let start = -max;
    let end = max + 1;
    for x in start..end {
      for y in start..end {
        for z in start..end {
          tmp[0] = key[0] + x;
          tmp[1] = key[1] + y;
          tmp[2] = key[2] + z;


          if min == 0 {
            if Utils::in_range(key, &tmp, max) {
              keys.push(tmp);
            }
          }

          if min > 0 {
            if !Utils::in_range(key, &tmp, min) &&
            Utils::in_range(key, &tmp, max) {
              keys.push(tmp);
            }
          }
        }
      }
    }

    keys
  }

  pub fn in_range(key1: &[i64; 3], key2: &[i64; 3], range: i64) -> bool {
    let mut dist_sqr = 0.0;
    for i in 0..key1.len() {
      let diff = key1[i] - key2[i];
      dist_sqr += (diff * diff) as f32;

    }

    // println!("{}: {}: {:?}", dist_sqr, (range as f32).powf(2.0), key2);
    dist_sqr <= (range as f32).powf(2.0)
  }

}






#[cfg(test)]
mod tests {
  use crate::{round, Utils};

  #[test]
  fn test_nearest_negative_positions_by_4_0() -> Result<(), String> {
    let scale = 4.0;
    assert_eq!(round(-0.1,   scale), 0.0);
    assert_eq!(round(-1.99,  scale), 0.0);
    assert_eq!(round(-2.0,   scale),-4.0);
    assert_eq!(round(-3.99,  scale),-4.0);

    assert_eq!(round(-4.1,   scale),-4.0);
    assert_eq!(round(-5.99,  scale),-4.0);
    assert_eq!(round(-6.1,   scale),-8.0);
    assert_eq!(round(-7.99,  scale),-8.0);
    Ok(())
  }

  #[test]
  fn test_nearest_positive_positions_by_4_0() -> Result<(), String> {
    let scale = 4.0;
    assert_eq!(round(0.1,   scale), 0.0);
    assert_eq!(round(1.99,  scale), 0.0);
    assert_eq!(round(2.0,   scale), 4.0);
    assert_eq!(round(3.99,  scale), 4.0);

    assert_eq!(round(4.1,   scale), 4.0);
    assert_eq!(round(5.99,  scale), 4.0);
    assert_eq!(round(6.1,   scale), 8.0);
    assert_eq!(round(7.99,  scale), 8.0);
    Ok(())
  }


  #[test]
  fn test_nearest_negative_positions_by_2_0() -> Result<(), String> {
    let scale = 2.0;
    assert_eq!(round(-0.1,   scale), 0.0);
    assert_eq!(round(-0.99,  scale), 0.0);
    assert_eq!(round(-1.0,   scale),-2.0);
    assert_eq!(round(-1.99,  scale),-2.0);

    assert_eq!(round(-2.1,   scale),-2.0);
    assert_eq!(round(-2.99,  scale),-2.0);
    assert_eq!(round(-3.1,   scale),-4.0);
    assert_eq!(round(-3.99,  scale),-4.0);
    Ok(())
  }

  #[test]
  fn test_nearest_positive_positions_by_2_0() -> Result<(), String> {
    let scale = 2.0;
    assert_eq!(round(0.1,   scale), 0.0);
    assert_eq!(round(0.99,  scale), 0.0);
    assert_eq!(round(1.0,   scale), 2.0);
    assert_eq!(round(1.99,  scale), 2.0);

    assert_eq!(round(2.1,   scale), 2.0);
    assert_eq!(round(2.99,  scale), 2.0);
    assert_eq!(round(3.1,   scale), 4.0);
    assert_eq!(round(3.99,  scale), 4.0);
    Ok(())
  }


  #[test]
  fn test_nearest_negative_positions_by_1_0() -> Result<(), String> {
    let scale = 1.0;
    assert_eq!(round(-0.1,   scale), 0.0);
    assert_eq!(round(-0.49,  scale), 0.0);
    assert_eq!(round(-0.5,   scale),-1.0);
    assert_eq!(round(-0.99,  scale),-1.0);

    assert_eq!(round(-1.3,   scale),-1.0);
    assert_eq!(round(-1.49,  scale),-1.0);
    assert_eq!(round(-1.5,   scale),-2.0);
    assert_eq!(round(-1.99,  scale),-2.0);
    Ok(())
  }


  #[test]
  fn test_nearest_positive_positions_by_1_0() -> Result<(), String> {
    let scale = 1.0;
    assert_eq!(round(0.1,   scale), 0.0);
    assert_eq!(round(0.49,  scale), 0.0);
    assert_eq!(round(0.5,   scale), 1.0);
    assert_eq!(round(0.99,  scale), 1.0);

    assert_eq!(round(1.1,   scale), 1.0);
    assert_eq!(round(1.49,  scale), 1.0);
    assert_eq!(round(1.5,   scale), 2.0);
    assert_eq!(round(1.99,  scale), 2.0);
    Ok(())
  }

  #[test]
  fn test_nearest_negative_positions_by_0_5() -> Result<(), String> {
    let scale = 0.5;
    assert_eq!(round(-0.1,   scale), 0.0);
    assert_eq!(round(-0.24,  scale), 0.0);
    assert_eq!(round(-0.25,  scale),-0.5);
    assert_eq!(round(-0.49,  scale),-0.5);
    
    assert_eq!(round(-0.6,   scale),-0.5);
    assert_eq!(round(-0.749, scale),-0.5);
    assert_eq!(round(-0.75,  scale),-1.0);
    assert_eq!(round(-0.99,  scale),-1.0);

    assert_eq!(round(-1.1,   scale),-1.0);
    assert_eq!(round(-1.24,  scale),-1.0);
    assert_eq!(round(-1.25,  scale),-1.5);
    assert_eq!(round(-1.49,  scale),-1.5);
    
    assert_eq!(round(-1.6,   scale),-1.5);
    assert_eq!(round(-1.749, scale),-1.5);
    assert_eq!(round(-1.75,  scale),-2.0);
    assert_eq!(round(-1.99,  scale),-2.0);
    Ok(())
  }

  #[test]
  fn test_nearest_positive_positions_by_0_5() -> Result<(), String> {
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
    Ok(())
  }

  #[test]
  fn test_nearest_negative_positions_by_0_25() -> Result<(), String> {
    let scale = 0.25;
    assert_eq!(round(-0.1,   scale), 0.0);
    assert_eq!(round(-0.124, scale), 0.0);
    assert_eq!(round(-0.125, scale),-0.25);
    assert_eq!(round(-0.249, scale),-0.25);

    assert_eq!(round(-0.26,  scale),-0.25);
    assert_eq!(round(-0.374, scale),-0.25);
    assert_eq!(round(-0.375, scale),-0.5);
    assert_eq!(round(-0.499, scale),-0.5);

    assert_eq!(round(-1.1,   scale),-1.0);
    assert_eq!(round(-1.124, scale),-1.0);
    assert_eq!(round(-1.125, scale),-1.25);
    assert_eq!(round(-1.249, scale),-1.25);

    assert_eq!(round(-1.26,  scale),-1.25);
    assert_eq!(round(-1.374, scale),-1.25);
    assert_eq!(round(-1.375, scale),-1.5);
    assert_eq!(round(-1.499, scale),-1.5);

    Ok(())
  }

  #[test]
  fn test_nearest_positive_positions_by_0_25() -> Result<(), String> {
    let scale = 0.25;
    assert_eq!(round(0.1,   scale), 0.0);
    assert_eq!(round(0.124, scale), 0.0);
    assert_eq!(round(0.125, scale), 0.25);
    assert_eq!(round(0.249, scale), 0.25);

    assert_eq!(round(0.26,  scale), 0.25);
    assert_eq!(round(0.374, scale), 0.25);
    assert_eq!(round(0.375, scale), 0.5);
    assert_eq!(round(0.499, scale), 0.5);

    assert_eq!(round(1.1,   scale), 1.0);
    assert_eq!(round(1.124, scale), 1.0);
    assert_eq!(round(1.125, scale), 1.25);
    assert_eq!(round(1.249, scale), 1.25);

    assert_eq!(round(1.26,  scale), 1.25);
    assert_eq!(round(1.374, scale), 1.25);
    assert_eq!(round(1.375, scale), 1.5);
    assert_eq!(round(1.499, scale), 1.5);

    Ok(())
  }

  #[test]
  fn test_get_tile_range() -> Result<(), String> {
    
    let range = Utils::get_tile_range(&[0, 0, 0], &[0, 0, 0]);
    assert_eq!(range, 0);

    let range = Utils::get_tile_range(&[0, 0, 0], &[1, 0, 0]);
    assert_eq!(range, 1);

    let range = Utils::get_tile_range(&[0, 0, 0], &[1, 1, 1]);
    assert_eq!(range, 1);

    let range = Utils::get_tile_range(&[0, 0, 0], &[2, 1, 1]);
    assert_eq!(range, 2);

    let range = Utils::get_tile_range(&[-1, -1, -1], &[0, 0, 0]);
    assert_eq!(range, 1);

    let range = Utils::get_tile_range(&[-2, -2, -2], &[1, 1, 1]);
    assert_eq!(range, 3);

    let range = Utils::get_tile_range(&[-3, -3, -3], &[-1, -1, -1]);
    assert_eq!(range, 2);

    let range = Utils::get_tile_range(&[-3, -3, -3], &[0, 0, -1]);
    assert_eq!(range, 3);

    Ok(())
  }

  #[test]
  fn test_get_keys_by_tile_dist_min0_max1() -> Result<(), String> {
    let start_key = [0, 0, 0];
    let min = 0;
    let max = 1;
    let keys = Utils::get_keys_by_tile_dist(&start_key, min, max);

    assert_eq!(keys.len(), 27);
    Ok(())
  }

  #[test]
  fn test_get_keys_by_tile_dist_min2_max3() -> Result<(), String> {
    let start_key = [0, 0, 0];
    let min = 2;
    let max = 3;
    let keys = Utils::get_keys_by_tile_dist(&start_key, min, max);

    for key in keys.iter() {
      // println!("{:?}", key);
      let range = Utils::get_tile_range(&start_key, key);
      assert!(range >= min && range <= max, "{:?} is out of range", key);
    }

    Ok(())
  }

  #[test]
  fn test_get_keys_by_tile_dist_min4_max6() -> Result<(), String> {
    let start_key = [0, 0, 0];
    let min = 4;
    let max = 6;
    let keys = Utils::get_keys_by_tile_dist(&start_key, min, max);

    for key in keys.iter() {
      println!("{:?}", key);
      let range = Utils::get_tile_range(&start_key, key);
      assert!(range >= min && range <= max, "{:?} is out of range", key);
    }

    Ok(())
  }

  #[test]
  fn test_in_range() -> Result<(), String> {
    assert!(Utils::in_range(&[0, 0, 0], &[0, 0, 0], 1));
    assert!(Utils::in_range(&[0, 0, 0], &[1, 0, 0], 1));

    assert!(Utils::in_range(&[0, 0, 0], &[1, 1, 0], 2));
    assert!(!Utils::in_range(&[0, 0, 0], &[1, 1, 0], 1));
    
    assert!(Utils::in_range(&[0, 0, 0], &[1, 1, 1], 2));
    assert!(!Utils::in_range(&[0, 0, 0], &[1, 1, 1], 1));

    assert!(Utils::in_range(&[0, 0, 0], &[2, 0, 0], 2));
    assert!(!Utils::in_range(&[0, 0, 0], &[2, 0, 0], 1));

    assert!(Utils::in_range(&[0, 0, 0], &[2, 2, 0], 3));
    assert!(!Utils::in_range(&[0, 0, 0], &[2, 2, 0], 2));

    Ok(())
  }

  #[test]
  fn test_get_keys_by_dist() -> Result<(), String> {
    let keys = Utils::get_keys_by_dist(&[0, 0, 0], 0, 1);
    assert_eq!(keys.len(), 7);

    // let keys = Utils::get_keys_by_dist(&[0, 0, 0], 0, 2);
    // assert_eq!(keys.len(), 7);

    // for k in keys.iter() {
    //   println!("{:?}", k);
    // }

    Ok(())
  }

}

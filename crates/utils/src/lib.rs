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

  pub fn get_delta_keys_by_lod(
    ranges: &Vec<u32>,
    prev_key: &[i64; 3],
    key: &[i64; 3],
    lod: usize, 
  ) -> Vec<[i64; 3]> {
    let keys = Utils::get_keys_by_lod(ranges, key, lod);
    let mut delta = Vec::new();
    for k in keys.iter() {
      // if Utils::get_tile_range(prev_key, k) > max {
      if !Utils::in_range_by_lod(prev_key, k, ranges, lod) {
        delta.push(*k);
      }
    }
    delta
  }

  pub fn in_range_by_lod(
    key1: &[i64; 3], 
    key2: &[i64; 3],
    ranges: &Vec<u32>,
    lod: usize
  ) -> bool {
    let min = ranges[lod] as i64;
    let max = ranges[lod + 1] as i64;

    if lod == 0 {
      return Utils::get_tile_range(key1, key2) <= max
    }

    if lod == 1 {
      return Utils::get_tile_range(key1, key2) > min &&
      Utils::in_range(key1, key2, max)
    }

    !Utils::in_range(key1, key2, min) && Utils::in_range(key1, key2, max)
  }

  pub fn get_keys_by_lod(
    ranges: &Vec<u32>,
    key: &[i64; 3], 
    lod: usize,
  ) -> Vec<[i64; 3]> {
    assert!(lod + 1 <= ranges.len());

    let min = ranges[lod] as i64;
    let max = ranges[lod + 1] as i64;
  
    if lod == 0 {
      return Utils::get_keys_by_tile_dist(key, min, max);
    }
  
    if lod == 1 {
      let keys = Utils::get_keys_by_dist(key, min, max);

      let mut res = Vec::new();
      for k in keys.iter() {
        if !Utils::in_range_by_lod(&key, k, &ranges, 0) {
          if !res.contains(k) {
            res.push(*k);
          }
        }
      }
      return res;
    }
  
    Utils::get_keys_by_dist(&key, min, max)
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


    let key = [0, 0, 0];
    let ranges = vec![0, 1, 4, 8, 12];
    for lod in 0..ranges.len() - 1 {
      let keys = Utils::get_keys_by_lod(&ranges, &key, lod);
      for k in keys.iter() {
        assert!(Utils::in_range_by_lod(&key, k, &ranges, lod));
      }
    }
    

    Ok(())
  }

  #[test]
  fn test_get_keys_by_dist() -> Result<(), String> {
    let keys = Utils::get_keys_by_dist(&[0, 0, 0], 0, 1);
    assert_eq!(keys.len(), 7);

    Ok(())
  }

  /// TODO: Add each lod index total keys if needed
  #[test]
  fn test_get_keys_by_lod() -> Result<(), String> {
    let key = [0, 0, 0];
    let ranges = vec![0, 1, 4, 8, 12];

    let mut all_keys = Vec::new();
    for lod in 0..ranges.len() - 1 {
      let keys = Utils::get_keys_by_lod(&ranges, &key, lod);
      for k in keys.iter() {
        if !all_keys.contains(k) {
          all_keys.push(*k);
        } else {
          assert!(false, "Duplicate key {:?} in lod {}", k, lod);
        }
      }
    }

    assert_eq!(all_keys.len(), 7153); // Set based on visual check
    Ok(())
  }

  #[test]
  fn test_get_delta_keys_by_lod() -> Result<(), String> {
    let prev_key = [0, 0, 0];
    let key = [1, 0, 0];
    let ranges = vec![0, 1, 4, 8, 12];

    /* Lod 0 */
    let keys0 = Utils::get_keys_by_lod(&ranges, &key, 0);
    let mut current_keys0 = Vec::new();

    // Remove keys outside range
    for k in keys0.iter() {
      if Utils::in_range_by_lod(&prev_key, &k, &ranges, 0) {
        current_keys0.push(*k);
      }
    }

    // Add the new keys
    let delta_keys0 = Utils::get_delta_keys_by_lod(&ranges, &prev_key, &key, 0);
    current_keys0.append(&mut delta_keys0.clone());

    assert_eq!(current_keys0.len(), keys0.len());

    // Check for duplicates by ensuring all keys are unique
    for k in keys0.iter() {
      assert!(current_keys0.contains(k));
    }


    /* Lod 1 */
    let lod1 = 1;
    let keys1 = Utils::get_keys_by_lod(&ranges, &key, lod1);
    let mut current_keys1 = Vec::new();

    // Remove keys outside range
    for k in keys1.iter() {
      if Utils::in_range_by_lod(&prev_key, &k, &ranges, lod1) {
        current_keys1.push(*k);
      }
    }

    // Add the new keys
    let delta_keys1 = Utils::get_delta_keys_by_lod(&ranges, &prev_key, &key, lod1);
    current_keys1.append(&mut delta_keys1.clone());
    current_keys1.append(&mut delta_keys1.clone());

    assert_eq!(current_keys1.len(), keys1.len());


    Ok(())
  }



}

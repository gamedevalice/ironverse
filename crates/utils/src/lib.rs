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

  // println!(
  //   "num {}, div {}, half {} base_div {}, base_val {}, modulus {}", 
  //   num, div, half, base_div, base_val, modulus
  // );
  res
}

#[cfg(test)]
mod tests {
  use crate::round;

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



}

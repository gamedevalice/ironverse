use bevy::prelude::*;
use voxels::chunk::chunk_manager::{ChunkManager, Chunk};
use crate::BevyVoxelResource;

pub fn set_voxel(
  chunk_manager: &mut ChunkManager,
  pos: Vec3,
  voxel: u8,
) {
  let mul = 1.0 / chunk_manager.voxel_scale;
  // let mul = 1.0;
  let p = [
    (pos.x * mul) as i64,
    (pos.y * mul) as i64,
    (pos.z * mul) as i64,
  ];

  chunk_manager.set_voxel2(&p, voxel);
}


pub fn load_chunk(resource: &mut BevyVoxelResource, key: [i64; 3]) -> Chunk {
  let res = resource.chunk_manager.get_chunk(&key);
  if res.is_none() {
    let chunk = resource.chunk_manager.new_chunk3(&key, resource.chunk_manager.depth as u8);
    resource.chunk_manager.set_chunk(&key, &chunk);
    return chunk;
  }

  res.unwrap().clone()
}


pub fn get_near_positions(pos: Vec3, unit: f32) -> Vec<Vec3> {
  let mut res = Vec::new();
  let min = -1;
  let max = 2;
  for x in min..max {
    for y in min..max {
      for z in min..max {
        res.push(Vec3::new(
          pos[0] + (x as f32 * unit),
          pos[1] + (y as f32 * unit),
          pos[2] + (z as f32 * unit),
        ));
      }
    }
  }

  res
}


pub fn get_key(pos: Vec3, voxel_scale: f32, seamless_size: u32) -> [i64; 3] {
  let p = [
    pos.x as i64,
    pos.y as i64,
    pos.z as i64,
  ];

  let div = (1.0 / voxel_scale) as u32;
  let s = seamless_size / div;

  let s1 = (seamless_size as f32) / (1.0 / voxel_scale);
  pos_to_key(pos, s1)
}


pub fn pos_to_key(pos: Vec3, seamless_size: f32) -> [i64; 3] {
  let mut x = pos[0];
  let mut y = pos[1];
  let mut z = pos[2];

  // Between -0.epsilon to -seamless_size..., it should be -1
  if x < 0.0 {
    x -= seamless_size;
  }
  if y < 0.0 {
    y -= seamless_size;
  }
  if z < 0.0 {
    z -= seamless_size;
  }

  [
    (x / seamless_size) as i64,
    (y / seamless_size) as i64,
    (z / seamless_size) as i64,
  ]
}


#[cfg(test)]
mod tests {
  use bevy::prelude::Vec3;
  use voxels::chunk::chunk_manager::ChunkManager;
  use crate::util::get_key;
  use super::get_near_positions;

  #[test]
  fn test_near_positions_1_0() -> Result<(), String> {
    let scale = 1.0;
    let pos = Vec3::new(0.0, 0.0, 0.0);
    let res = get_near_positions(pos, scale);
    let expected = vec![
      Vec3::new(-1.0, -1.0, -1.0),
      Vec3::new(-1.0, -1.0, 0.0),
      Vec3::new(-1.0, -1.0, 1.0),
      Vec3::new(-1.0, 0.0, -1.0),
      Vec3::new(-1.0, 0.0, 0.0),
      Vec3::new(-1.0, 0.0, 1.0),
      Vec3::new(-1.0, 1.0, -1.0),
      Vec3::new(-1.0, 1.0, 0.0),
      Vec3::new(-1.0, 1.0, 1.0),
      Vec3::new(0.0, -1.0, -1.0),
      Vec3::new(0.0, -1.0, 0.0),
      Vec3::new(0.0, -1.0, 1.0),
      Vec3::new(0.0, 0.0, -1.0),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 1.0),
      Vec3::new(0.0, 1.0, -1.0),
      Vec3::new(0.0, 1.0, 0.0),
      Vec3::new(0.0, 1.0, 1.0),
      Vec3::new(1.0, -1.0, -1.0),
      Vec3::new(1.0, -1.0, 0.0),
      Vec3::new(1.0, -1.0, 1.0),
      Vec3::new(1.0, 0.0, -1.0),
      Vec3::new(1.0, 0.0, 0.0),
      Vec3::new(1.0, 0.0, 1.0),
      Vec3::new(1.0, 1.0, -1.0),
      Vec3::new(1.0, 1.0, 0.0),
      Vec3::new(1.0, 1.0, 1.0),
    ];
    
    for p in res.iter() {
      assert!(expected.contains(p));
    }
    Ok(())
  }


  #[test]
  fn test_near_positions_0_5() -> Result<(), String> {
    let scale = 0.5;
    let pos = Vec3::new(0.0, 0.0, 0.0);

    let res = get_near_positions(pos, scale);

    let expected = vec![
      Vec3::new(-0.5, -0.5, -0.5),
      Vec3::new(-0.5, -0.5, 0.0),
      Vec3::new(-0.5, -0.5, 0.5),
      Vec3::new(-0.5, 0.0, -0.5),
      Vec3::new(-0.5, 0.0, 0.0),
      Vec3::new(-0.5, 0.0, 0.5),
      Vec3::new(-0.5, 0.5, -0.5),
      Vec3::new(-0.5, 0.5, 0.0),
      Vec3::new(-0.5, 0.5, 0.5),
      Vec3::new(0.0, -0.5, -0.5),
      Vec3::new(0.0, -0.5, 0.0),
      Vec3::new(0.0, -0.5, 0.5),
      Vec3::new(0.0, 0.0, -0.5),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 0.5),
      Vec3::new(0.0, 0.5, -0.5),
      Vec3::new(0.0, 0.5, 0.0),
      Vec3::new(0.0, 0.5, 0.5),
      Vec3::new(0.5, -0.5, -0.5),
      Vec3::new(0.5, -0.5, 0.0),
      Vec3::new(0.5, -0.5, 0.5),
      Vec3::new(0.5, 0.0, -0.5),
      Vec3::new(0.5, 0.0, 0.0),
      Vec3::new(0.5, 0.0, 0.5),
      Vec3::new(0.5, 0.5, -0.5),
      Vec3::new(0.5, 0.5, 0.0),
      Vec3::new(0.5, 0.5, 0.5),
    ];
    
    for p in res.iter() {
      assert!(expected.contains(p));
    }
    
    Ok(())
  }


  #[test]
  fn test_get_key_1_0() -> Result<(), String> {
    let depth = 4;
    let voxel_scale = 1.0;
    let range = 1;
    let manager = ChunkManager::new(depth, voxel_scale, range, Vec::new());
    
    let pos = vec![
      Vec3::new(-27.9, -27.9, -27.9),
      Vec3::new(-13.9, -13.9, -13.9),
      Vec3::new(  0.0,   0.0,   0.0),
      Vec3::new( 14.0,  14.0,  14.0),
      Vec3::new( 28.0,  28.0,  28.0),
    ];
    
    let expected = vec![
      [-2,-2,-2],
      [-1,-1,-1],
      [ 0, 0, 0],
      [ 1, 1, 1],
      [ 2, 2, 2],
    ];
    for (i, p) in pos.iter().enumerate() {
      let key = get_key(*p, voxel_scale, manager.seamless_size());
      assert_eq!(key, expected[i]);
    }
    Ok(())
  }

  #[test]
  fn test_get_key_0_5() -> Result<(), String> {
    let depth = 4;
    let voxel_scale = 0.5;
    let range = 1;
    let manager = ChunkManager::new(depth, voxel_scale, range, Vec::new());
    
    let pos = vec![
      Vec3::new(-13.9, -13.9, -13.9),
      Vec3::new(-6.9,   -6.9,  -6.9),
      Vec3::new( 0.0,    0.0,   0.0),
      Vec3::new( 7.0,    7.0,   7.0),
      Vec3::new( 14.0,  14.0,  14.0),
    ];
    
    let expected = vec![
      [-2,-2,-2],
      [-1,-1,-1],
      [ 0, 0, 0],
      [ 1, 1, 1],
      [ 2, 2, 2],
    ];
    for (i, p) in pos.iter().enumerate() {
      let key = get_key(*p, voxel_scale, manager.seamless_size());
      assert_eq!(key, expected[i]);
    }
    Ok(())
  }

  #[test]
  fn test_get_key_0_25() -> Result<(), String> {
    let depth = 4;
    let voxel_scale = 0.25;
    let range = 1;
    let manager = ChunkManager::new(depth, voxel_scale, range, Vec::new());
    
    let pos = vec![
      Vec3::new(-6.9, -6.9, -6.9),
      Vec3::new(-3.4, -3.4, -3.4),
      Vec3::new( 0.0,  0.0,  0.0),
      Vec3::new( 3.5,  3.5,  3.5),
      Vec3::new( 7.0,  7.0,  7.0),
    ];
    
    let expected = vec![
      [-2,-2,-2],
      [-1,-1,-1],
      [ 0, 0, 0],
      [ 1, 1, 1],
      [ 2, 2, 2],
    ];
    for (i, p) in pos.iter().enumerate() {
      let key = get_key(*p, voxel_scale, manager.seamless_size());
      assert_eq!(key, expected[i]);
    }
    Ok(())
  }

}

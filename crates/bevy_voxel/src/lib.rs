mod physics;
mod remove;
mod add;
mod util;
mod functions;
mod implement;

use bevy::prelude::*;
use physics::Physics;
use rapier3d::prelude::ColliderHandle;
use voxels::{chunk::chunk_manager::{ChunkManager, Chunk}, data::voxel_octree::MeshData};

pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(functions::CustomPlugin)
      .add_plugin(remove::CustomPlugin)
      .add_plugin(add::CustomPlugin);
  }
}

#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
  pub physics: Physics,

  colliders_cache: Vec<ColliderHandle>,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
    }
  }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum EditState {
  #[default]
  AddNormal,
  AddSnap,
  RemoveNormal,
  RemoveSnap,
}

#[derive(Component, Clone)]
pub struct Selected {
  pub pos: Option<Vec3>,
}

impl Default for Selected {
  fn default() -> Self {
    Self {
      pos: None,
    }
  }
}

#[derive(Component, Clone)]
pub struct Preview {
  pub pos: Option<Vec3>,
  pub level: u8,
  pub size: u8,
  pub voxel: u8,
}

impl Default for Preview {
  fn default() -> Self {
    let level = 1;
    Self {
      pos: None,
      level: level,
      size: 2_u8.pow(level as u32),
      voxel: 1,
    }
  }
}
#[derive(Component, Clone)]
pub struct SelectedGraphics;

#[derive(Component, Clone)]
pub struct PreviewGraphics;



#[derive(Component, Debug, Clone)]
pub struct ChunkData {
  pub key: [i64; 3],
  pub data: MeshData,
}

#[derive(Component, Debug, Clone)]
pub struct Chunks {
  pub data: Vec<ChunkData>,
}

impl Default for Chunks {
  fn default() -> Self {
    Self {
      data: Vec::new(),
    }
  }
}

#[derive(Component)]
pub struct Center {
  pub key: [i64; 3],
}

impl Default for Center {
  fn default() -> Self {
    Self {
      key: [0; 3],
    }
  }
}

#[derive(Default, Component)]
pub struct ChunkEdit {
  pub position: Option<Vec3>,
  pub chunk: Option<Chunk>,
}

#[derive(Component)]
pub struct ChunkEditParams {
  pub level: u8,
  pub dist: f32,
  pub voxel: u8,

  pub size: u32,
}

impl Default for ChunkEditParams {
  fn default() -> Self {
    let level = 1;
    Self {
      level: level,
      dist: 8.0,
      voxel: 0,
      size: 2_u32.pow(level as u32),
    }
  }
}

#[cfg(test)]
mod tests {
  use bevy::prelude::Vec3;
  use voxels::chunk::chunk_manager::ChunkManager;
  use crate::{get_key, get_near_positions};

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


/*
  Implement changable voxel edit size
    Preview
    Edit
*/
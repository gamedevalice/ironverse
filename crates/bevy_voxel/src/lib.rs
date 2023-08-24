mod physics;
mod util;
mod functions;
mod implement;
mod editstate;

use bevy::prelude::*;
use physics::Physics;
use rapier3d::prelude::ColliderHandle;
use voxels::{chunk::chunk_manager::{ChunkManager, Chunk}, data::voxel_octree::MeshData};

pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<EditState>()
      .add_state::<ShapeState>()
      .add_plugin(functions::CustomPlugin)
      .add_plugin(editstate::CustomPlugin);
  }
}

#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
  pub physics: Physics,

  colliders_cache: Vec<ColliderHandle>,
  shape_state: ShapeState,
  edit_state: EditState,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
      shape_state: ShapeState::Cube,
      edit_state: EditState::AddNormal,
    }
  }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum EditState {
  AddNormal,
  AddDist,
  AddSnap,
  
  #[default]
  RemoveNormal,
  RemoveDist,
  RemoveSnap,
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum ShapeState {
  #[default]
  Cube,
  Sphere,
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

  pub sphere_size: f32,
  pub dist: f32,
}

impl Default for Preview {
  fn default() -> Self {
    let level = 1;
    Self {
      pos: None,
      level: level,
      size: 2_u8.pow(level as u32),
      voxel: 1,

      sphere_size: 1.0,
      dist: 3.0,
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


/*
  Implement changable voxel edit size
    Preview
    Edit
*/
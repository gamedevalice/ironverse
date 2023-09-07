mod physics;
mod util;
mod functions;
mod implement;
mod editstate;
mod lod;

use bevy::{prelude::*, utils::HashMap};
use physics::Physics;
use rapier3d::prelude::ColliderHandle;
use voxels::{chunk::chunk_manager::{ChunkManager, Chunk}, data::voxel_octree::MeshData};

use cfg_if::cfg_if;


pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<EditState>()
      .add_state::<ShapeState>()
      .add_plugin(functions::CustomPlugin)
      .add_plugin(editstate::CustomPlugin)
      .add_plugin(lod::CustomPlugin);

    cfg_if! {
      if #[cfg(target_arch = "wasm32")] {
        app
          .add_plugin(multithread::plugin::CustomPlugin);
      }
    }
  }
}

#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
  pub physics: Physics,

  colliders_cache: Vec<ColliderHandle>,
  shape_state: ShapeState,
  edit_state: EditState,
  ranges: Vec<u8>,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
      shape_state: ShapeState::Cube,
      edit_state: EditState::AddNormal,
      ranges: vec![0, 1, 3, 5, 7],
    }
  }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum EditState {
  #[default]
  AddNormal,
  AddDist,
  AddSnap,
  
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



#[derive(Component, Debug, Clone, Default)]
pub struct MeshComponent {
  pub data: HashMap<[i64; 3], MeshData>,
  pub added_keys: Vec<[i64; 3]>,
}

#[derive(Component, Debug, Clone)]
pub struct Chunks {
  pub data: HashMap<[i64; 3], Chunk>,
  pub added_keys: Vec<[i64; 3]>,
}

impl Default for Chunks {
  fn default() -> Self {
    Self {
      data: HashMap::new(),
      added_keys: Vec::new(),
    }
  }
}

#[derive(Component)]
pub struct Center {
  pub prev_key: [i64; 3],
  pub key: [i64; 3],
}

impl Default for Center {
  fn default() -> Self {
    Self {
      prev_key: [0; 3],
      key: [0; 3],
    }
  }
}


pub struct ChunkMesh {
  pub key: [i64; 3],
  pub mesh: MeshData,
}
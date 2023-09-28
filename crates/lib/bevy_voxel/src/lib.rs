mod physics;
mod util;
mod functions;
mod implement;
pub mod editstate;
mod lod;

use bevy::{prelude::*, utils::HashMap};
use flume::{Sender, Receiver};
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
      .add_plugins(functions::CustomPlugin)
      .add_plugins(editstate::CustomPlugin)
      .add_plugins(lod::CustomPlugin);

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

  pub send_key: Sender<([i64; 3], usize)>,
  pub recv_key: Receiver<([i64; 3], usize)>,

  pub send_chunk: Sender<Chunk>,
  pub recv_chunk: Receiver<Chunk>,

  pub send_process_mesh: Sender<Chunk>,
  pub recv_process_mesh: Receiver<Chunk>,

  pub send_mesh: Sender<MeshData>,
  pub recv_mesh: Receiver<MeshData>,

  colliders_cache: Vec<ColliderHandle>,
  shape_state: ShapeState,
  edit_state: EditState,
  pub ranges: Vec<u32>,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    let (send_key, recv_key) = flume::unbounded();
    let (send_chunk, recv_chunk) = flume::unbounded();
    let (send_process_mesh, recv_process_mesh) = flume::unbounded();
    let (send_mesh, recv_mesh) = flume::unbounded();

    Self {
      chunk_manager: ChunkManager::default(),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
      shape_state: ShapeState::Cube,
      edit_state: EditState::AddNormal,
      ranges: vec![0, 1, 3, 5, 7],

      send_key: send_key,
      recv_key: recv_key,
      send_chunk: send_chunk,
      recv_chunk: recv_chunk,
      send_process_mesh: send_process_mesh,
      recv_process_mesh: recv_process_mesh,
      send_mesh: send_mesh,
      recv_mesh: recv_mesh,
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
      dist: 8.0,
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
  pub added: Vec<(MeshData, ColliderHandle)>,
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
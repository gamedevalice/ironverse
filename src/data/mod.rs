use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};
use voxels::chunk::chunk_manager::{ChunkManager, Chunk};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<GameState>()
      .add_state::<CursorState>()
      .insert_resource(GameResource::default())
      .insert_resource(UIResource::default())
      .add_state::<UIState>();
  }
}

#[derive(Resource)]
pub struct GameResource {
  pub chunk_manager: ChunkManager,
  pub data: Data,

  pub preview_chunk_manager: ChunkManager,
  pub modified_chunks: HashMap<[i64; 3], Chunk>,
  pub export_obj: Option<String>,

  pub colors: Vec<[f32; 3]>,
  pub voxel_scale: f32,
}

impl Default for GameResource {
  fn default() -> Self {
    let colors = vec![
      [0.2, 0.2, 0.2], 
      [1.0, 0.0, 0.0], 
      [0.0, 1.0, 0.0], 
      [0.0, 0.0, 1.0],

      [0.2, 0.0, 0.0],
      [0.4, 0.0, 0.0],
      [0.6, 0.0, 0.0],
      [0.8, 0.0, 0.0],

      [0.0, 0.2, 0.0],
      [0.0, 0.4, 0.0],
    ];

    Self {
      chunk_manager: ChunkManager::default(),
      data: Data::default(),
      preview_chunk_manager: ChunkManager::default(),
      modified_chunks: HashMap::new(),
      export_obj: None,
      colors: colors,
      voxel_scale: 0.25,
    }
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
  // Start,
  #[default]
  Start,
  Load,
  Init,

  Play,
  Pause,
  End,
  New,
  LoadGame,
  SaveGame,
  
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct Update;


#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum CursorState {
  #[default]
  None,
  Locked,
}



#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Data {
  pub status: Status,
  pub terrains: Terrains,
}

impl Default for Data {
  fn default() -> Self {
    Self {
      status: Status { position: [0.0, 5.0, 0.0] },
      terrains: Terrains { keys: Vec::new(), voxels: Vec::new() }
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Status {
  pub position: [f32; 3]
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Terrains {
  pub keys: Vec<[i64; 3]>,
  pub voxels: Vec<String>,
}


#[derive(Resource)]
pub struct UIResource {
  pub load_file_path: String,
  pub load_file_init: bool,   // Have to change later after updating bevy version
  pub total_materials: u8,
}

impl Default for UIResource {
  fn default() -> Self {
    Self {
      load_file_path: "".to_string(),
      load_file_init: true,
      total_materials: 16,
    }
  }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum UIState {
  #[default]
  Default,
  Menu,
  New,
  Restarting,
  Load,
  Save,

  // #[default]
  Inventory,
}





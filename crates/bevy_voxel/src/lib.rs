use bevy::prelude::*;
use voxels::chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys};

pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(BevyVoxelResource::default())
      .add_startup_system(startup);
  }
}

fn startup() {
  println!("startup BevyVoxel");
}



#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
    }
  }
}

impl BevyVoxelResource {
  pub fn load_adj_chunks(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    // let keys = adjacent_keys(&key, 1, true);
    // for key in keys.iter() {
    //   let res = self.chunk_manager.get_chunk(key);
    //   if res.is_none() {
    //     self.chunk_manager.new_chunk3(key, self.chunk_manager.depth);
    //   }
    // }


    chunks
  }
}


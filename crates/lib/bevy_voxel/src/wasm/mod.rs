use bevy::prelude::*;
use multithread::plugin::{PluginResource, send_key, Key, send_chunk};
use voxels::chunk::chunk_manager::ChunkManager;
use crate::BevyVoxelResource;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_systems(Update, (
        recv_keys,
        recv_chunk,
        load_mesh
      ));
  }
}

#[derive(Resource)]
struct LocalResource {
  duration: f32,
  keys_count: usize,
  keys_total: usize,
  done: bool,
  manager: ChunkManager,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      duration: 0.0,
      keys_count: 0,
      keys_total: 0,
      done: true,
      manager: ChunkManager::default(),
    }
  }
}


#[derive(Component)]
pub struct ChunkGraphics;

fn recv_keys(
  mut commands: Commands,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  //let thread_pool = AsyncComputeTaskPool::get();

  let depth = bevy_voxel_res.chunk_manager.depth as u8;
  let noise = bevy_voxel_res.chunk_manager.noise;

  for (key, lod) in bevy_voxel_res.recv_key.drain() {
    let key = key.clone();
    send_key(Key {
      key: key,
      lod: lod
    });
  } 
}

fn recv_chunk(
  plugin_res: Res<PluginResource>,
  mut commands: Commands,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for chunk in plugin_res.recv_chunk.drain() {
    // info!("update() {:?}", bytes);
    // info!("wasm_recv_data");
    //local_res.keys_count += 1;
    
    // let octree: Octree = bincode::deserialize(&bytes[..]).unwrap();
    // let chunk = Chunk {
    //   key: octree.key,
    //   octree: VoxelOctree::new_from_bytes(octree.data),
    //   ..Default::default()
    // };

    send_chunk(chunk);
  }
  
}

fn load_mesh(
  plugin_res: Res<PluginResource>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for data in plugin_res.recv_mesh.drain() {
    // info!("wasm_recv_mesh {:?}", data.key);

    bevy_voxel_res.send_mesh.send(data);
  }
}

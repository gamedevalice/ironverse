use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use voxels::{chunk::chunk_manager::{ChunkManager, Chunk}, data::{voxel_octree::{VoxelMode, MeshData}, surface_nets::VoxelReuse}};
use futures_lite::future;
use crate::BevyVoxelResource;


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, recv_keys)
      .add_systems(Update, recv_chunk)
      .add_systems(Update, recv_process_mesh)
      .add_systems(Update, recv_mesh);
  }
}


fn recv_keys(
  mut commands: Commands,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  let thread_pool = AsyncComputeTaskPool::get();

  let depth = bevy_voxel_res.chunk_manager.depth as u8;
  let noise = bevy_voxel_res.chunk_manager.noise;

  for (key, lod) in bevy_voxel_res.recv_key.drain() {
    let key = key.clone();
    let task = thread_pool.spawn(async move {
      let chunk = ChunkManager::new_chunk(&key, depth, lod, noise);
      chunk
    });
  
    // Spawn new entity and add our new task as a component
    commands.spawn(LoadChunk(task));
  }
  
}

fn recv_chunk(
  mut commands: Commands,
  bevy_voxel_res: Res<BevyVoxelResource>,
  mut tasks: Query<(Entity, &mut LoadChunk)>,
) {
  for (entity, mut task) in &mut tasks {
    if let Some(chunk) = future::block_on(future::poll_once(&mut task.0)) {
      let _ = bevy_voxel_res.send_chunk.send(chunk);

      // Task is complete, so remove task component from entity
      commands.entity(entity).remove::<LoadChunk>();
    }
  }
}


fn recv_process_mesh(
  mut commands: Commands,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  let thread_pool = AsyncComputeTaskPool::get();

  let depth = bevy_voxel_res.chunk_manager.depth;
  let scale = bevy_voxel_res.chunk_manager.voxel_scale;

  for chunk in bevy_voxel_res.recv_process_mesh.drain() {
    let colors = bevy_voxel_res.chunk_manager.colors.clone();
    let task = thread_pool.spawn(async move {
      chunk.octree.compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut VoxelReuse::new(depth, 3), 
        &colors, 
        scale, 
        chunk.key,
        chunk.lod
      )
    });
  
    // Spawn new entity and add our new task as a component
    commands.spawn(LoadMeshData(task));
  }
  
}

fn recv_mesh(
  mut commands: Commands,
  bevy_voxel_res: Res<BevyVoxelResource>,
  mut tasks: Query<(Entity, &mut LoadMeshData)>,
) {
  for (entity, mut task) in &mut tasks {
    if let Some(data) = future::block_on(future::poll_once(&mut task.0)) {
      let _ = bevy_voxel_res.send_mesh.send(data);

      // Task is complete, so remove task component from entity
      commands.entity(entity).remove::<LoadMeshData>();
    }
  }
}




#[derive(Component)]
struct LoadChunk(Task<Chunk>);

#[derive(Component)]
struct LoadMeshData(Task<MeshData>);


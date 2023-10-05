mod sphere;
mod cube;


use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::{prelude::*, utils::HashMap};

use rapier3d::prelude::ColliderHandle;
use utils::Utils;
use voxels::chunk::adjacent_keys;
use voxels::chunk::chunk_manager::ChunkManager;
use voxels::chunk::world_pos_to_key;
use voxels::data::voxel_octree::MeshData;
use voxels::data::voxel_octree::VoxelOctree;
use voxels::utils::key_to_world_coord_f32;
use crate::{BevyVoxelResource, Selected, Preview, Chunks, Center, ChunkData, ShapeState, EditState, MeshComponent};
use voxels::chunk::chunk_manager::Chunk;

use cfg_if::cfg_if;
cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
    use multithread::plugin::PluginResource;
    use multithread::plugin::send_key;
    use multithread::plugin::send_chunk;
    use multithread::plugin::Octree;
    use multithread::plugin::Key;
  }
}

cfg_if! {
  if #[cfg(not(target_arch = "wasm32"))] {
    mod async_loading;
  }
}


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(sphere::CustomPlugin)
      .add_plugins(cube::CustomPlugin)
      .insert_resource(BevyVoxelResource::default())
      .add_systems(Startup, startup)
      .add_systems(Update, update)
      .add_systems(Update, detect_selected_voxel_position)
      .add_systems(Update, load_main_chunks)
      .add_systems(Update, load_main_delta_chunks)
      .add_systems(Update, load_lod_chunks)
      .add_systems(Update, load_lod_center_changed)
      .add_systems(Update, receive_chunks)
      .add_systems(Update, receive_mesh)
      .add_systems(Update, shape_state_changed);

    cfg_if! {
      if #[cfg(not(target_arch = "wasm32"))] {
        app
          .add_plugins(async_loading::CustomPlugin);
      }
    }
    
    cfg_if! {
      if #[cfg(target_arch = "wasm32")] {
        app
        .insert_resource(LocalResource::default())
        .add_systems(Update, recv_keys)
        .add_systems(Update, recv_chunk)
        .add_systems(Update, load_mesh);
      }
    }
    
  }
}

cfg_if! {
  if #[cfg(target_arch = "wasm32")] {

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

  }
}

fn startup() {
  println!("startup BevyVoxel");
}

fn update(
  mut res: ResMut<BevyVoxelResource>,
  shape_state: Res<State<ShapeState>>,
  edit_state: Res<State<EditState>>,
) {
  res.physics.step();
  res.shape_state = *State::get(&shape_state);
  res.edit_state = *State::get(&edit_state);
}

fn detect_selected_voxel_position(
  mut cam: Query<(&Transform, &mut Selected), With<Selected>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut selected) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      if selected.pos.is_some() {
        selected.pos = None;
      }
      continue;
    }

    let pos = bevy_voxel_res.get_hit_voxel_pos(hit.unwrap());
    if pos.is_none() && selected.pos.is_some() {
      selected.pos = pos;
    }

    if pos.is_some() {
      if selected.pos.is_some() {
        let p = pos.unwrap();
        let current = selected.pos.unwrap();
        if current != p {
          selected.pos = pos;
        }
      }
      
      if selected.pos.is_none() {
        selected.pos = pos;
      }
    }
  }
}

fn load_main_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks, &mut MeshComponent), Added<Chunks>>
) {
  for (center, mut chunks, mut mesh_comp) in &mut chunks {
    let lod = 0;
    let keys = res.get_keys_by_lod(center.key, lod);

    let tmp_c = res.load_chunks(&keys, &chunks.data, lod);
    for c in tmp_c.iter() {
      chunks.data.insert(c.key, c.clone());
    }
    chunks.added_keys.append(&mut keys.clone());


    let data = res.load_mesh_data(&tmp_c);
    for (d, handle) in data.iter() {
      mesh_comp.data.insert(d.key, d.clone());
      mesh_comp.added.push((d.clone(), *handle));
    }
  }
}

fn load_lod_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks, &mut MeshComponent), Added<Chunks>>
) {
  for (center, mut chunks, mut mesh_comp) in &mut chunks {
    for lod in 1..res.ranges.len() - 1 {
      let keys = res.get_keys_by_lod(center.key, lod);
      request_load_chunk(&keys, &mut res, lod);
    }
  }
}

fn load_main_delta_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks, &mut MeshComponent), Changed<Center>>
) {
  for (center, mut chunks, mut mesh_comp) in &mut centers {
    let lod = 0;
    let keys = res.get_delta_keys_by_lod(
      &center.prev_key, &center.key, lod
    );

    let tmp_c = res.load_chunks(&keys, &chunks.data, lod);
    for c in tmp_c.iter() {
      chunks.data.insert(c.key, c.clone());
    }
    chunks.added_keys.clear();
    chunks.added_keys.append(&mut keys.clone());

    mesh_comp.added.clear();
    let data = res.load_mesh_data(&tmp_c);
    for (d, handle) in data.iter() {
      mesh_comp.data.insert(d.key, d.clone());
      mesh_comp.added.push((d.clone(), *handle));
    }
  }
}

fn load_lod_center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks, &mut MeshComponent), Changed<Center>>
) {
  
  for (center, mut chunks, mut mesh_comp) in &mut centers {
    for lod in 1..res.ranges.len() - 1 {
      let keys = res.get_delta_keys_by_lod(
        &center.prev_key, &center.key, lod
      );

      for key in keys.iter() {
        let d = chunks.data.get(key);
        if d.is_none() {
          let _ = res.send_key.send((*key, lod));
        }
        if d.is_some() {
          let mut data = d.unwrap().clone();
          data.lod = lod;
          res.send_process_mesh.send(data);
        }
      }
    }
  }
}





fn shape_state_changed(
  shape_state: Res<State<ShapeState>>,
  mut local: Local<ShapeState>,
  mut previews: Query<&mut Preview>,

  edit_state: Res<State<EditState>>,
  mut local1: Local<EditState>,
) {
  if *local != *State::get(&shape_state) {
    *local = *State::get(&shape_state);
    for mut preview in &mut previews {
      preview.size = preview.size;
    }
  }

  if *local1 != *State::get(&edit_state) {
    *local1 = *State::get(&edit_state);
    for mut preview in &mut previews {
      preview.size = preview.size;
    }
  }
  
}



fn request_load_chunk(
  keys: &Vec<[i64; 3]>,
  bevy_voxel_res: &mut BevyVoxelResource,
  lod: usize
) {
  for key in keys.iter() {
    let _ = bevy_voxel_res.send_key.send((*key, lod));
  }
}

fn receive_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut queries: Query<(&Center, &mut Chunks, &mut MeshComponent)>
) {
  for c in res.recv_chunk.drain() {
    for (center, mut chunks, mut mesh_comp) in &mut queries {
      res.send_process_mesh.send(c.clone());

      // let mut chunk = c.clone();
      // chunk.lod = 0;
      // chunks.data.insert(c.key, chunk);
    }
  }
}

fn receive_mesh(
  mut res: ResMut<BevyVoxelResource>,
  mut queries: Query<(&Center, &mut Chunks, &mut MeshComponent)>
) {
  let max_lod = res.chunk_manager.depth as u8;
  let ranges = res.ranges.clone();
  for data in res.recv_mesh.drain() {
    for (center, mut chunks, mut mesh_comp) in &mut queries {
      let d = data.clone();
      // mesh_comp.data.insert(d.key, d);

      if res.in_range_by_lod(&center.key, &data.key, data.lod) {
        if data.lod == 0 {
          // println!("Error: Lod 0 should not be loaded async");
        }

        if data.indices.len() > 0 {
          mesh_comp.added.push((data.clone(), ColliderHandle::invalid()));
        }
      }
    }
  }
}



fn get_keys_without_data(
  keys: &Vec<[i64; 3]>,
  data: &HashMap<[i64; 3], MeshData>
) -> Vec<[i64; 3]> {
  let mut filtered_keys = Vec::new();
  for k in keys.iter() {
    if !data.contains_key(k) {
      filtered_keys.push(*k);
    }
  }
  filtered_keys
}



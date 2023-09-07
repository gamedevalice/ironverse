mod sphere;
mod cube;

use bevy::prelude::*;
use utils::Utils;
use crate::{BevyVoxelResource, Selected, Preview, Chunks, Center, ChunkData, ShapeState, EditState, MeshComponent};

use cfg_if::cfg_if;
cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
    use multithread::plugin::PluginResource;
  }
}


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(sphere::CustomPlugin)
      .add_plugin(cube::CustomPlugin)
      .insert_resource(BevyVoxelResource::default())
      .add_startup_system(startup)
      .add_system(update)
      .add_system(detect_selected_voxel_position)
      .add_system(request_chunks)
      // .add_system(receive_chunks)
      .add_system(center_changed)
      .add_system(shape_state_changed);
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
  res.shape_state = shape_state.0;
  res.edit_state = edit_state.0;
}

fn detect_selected_voxel_position(
  mut cam: Query<(&Transform, &mut Selected), With<Selected>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut selected) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
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

fn request_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks, &mut MeshComponent), Added<Chunks>>
) {
  for (center, mut chunks, mut mesh_comp) in &mut chunks {
    /*
      Load data
      Load mesh
     */
    let lod = res.chunk_manager.depth as u8;
    let keys = res.get_keys_by_lod(center.key, lod);
    let tmp_c = res.load_chunks(&keys);
    for c in tmp_c.iter() {
      chunks.data.insert(c.key, c.clone());
    }
    let data = res.load_mesh_data(center.key, &tmp_c);
    chunks.added_keys.append(&mut keys.clone());

    for d in data.iter() {
      mesh_comp.data.insert(d.key, d.clone());
    }
    mesh_comp.added_keys.append(&mut keys.clone());

    // chunks.data.clear();

    // let all_chunks = res.load_adj_mesh_data(center.key);
    // for (key, data) in all_chunks.iter() {
    //   chunks.data.push(ChunkData {
    //     data: data.clone(),
    //     key: *key,
    //   });
    // }

    // let lod = res.chunk_manager.depth;

    // res.request_chunks(center.key, lod as u8 - 1);
    // res.request_chunks(center.key, lod as u8 - 2);
    // res.request_chunks(center.key, lod as u8 - 3);

    // let mut meshes = res.load_lod_meshes(center.key, lod as u8 - 1);
    // meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 2));
    // meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 3));
    // for mesh in meshes.iter() {
    //   chunks.data.push(ChunkData {
    //     data: mesh.mesh.clone(),
    //     key: mesh.key,
    //   });
    // }
  }
}

// fn receive_chunks(
//   mut res: ResMut<BevyVoxelResource>,
//   plugin_res: Res<PluginResource>,
// ) {

//   for chunk in plugin_res.recv_chunk.drain() {
    
//     res.chunk_manager.chunks.insert(chunk.key, chunk);
//   }

  
// }



fn center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks), Changed<Center>>
) {
  for (center, mut chunks) in &mut centers {
    let all_chunks = res.load_adj_mesh_data(center.key);
    chunks.data.clear();
  }
}


fn shape_state_changed(
  shape_state: Res<State<ShapeState>>,
  mut local: Local<ShapeState>,
  mut previews: Query<&mut Preview>,

  edit_state: Res<State<EditState>>,
  mut local1: Local<EditState>,
) {
  if *local != shape_state.0 {
    *local = shape_state.0;
    for mut preview in &mut previews {
      preview.size = preview.size;
    }
  }

  if *local1 != edit_state.0 {
    *local1 = edit_state.0;
    for mut preview in &mut previews {
      preview.size = preview.size;
    }
  }
  
}


/*
  Async loading
    Request Chunk to load by key
    Receive Chunk loaded by keys
    Request MeshData by chunk
    Receive MeshData
*/




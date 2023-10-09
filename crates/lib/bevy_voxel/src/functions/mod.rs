mod sphere;
mod cube;

use bevy::prelude::*;
use rapier3d::prelude::ColliderHandle;
use crate::{BevyVoxelResource, Selected, Preview, Chunks, Center, ShapeState, EditState, MeshComponent};

use cfg_if::cfg_if;

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
      .add_systems(Update, receive_chunks)
      .add_systems(Update, receive_mesh)
      .add_systems(Update, shape_state_changed)
      .add_systems(Update, (
        load_main_octrees,
        load_main_delta_octrees,
        load_lod_chunks,
        load_lod_delta_octrees
      ));

    cfg_if! {
      if #[cfg(not(target_arch = "wasm32"))] {
        app
          .add_plugins(async_loading::CustomPlugin);
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

fn load_main_octrees(
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
  chunks: Query<&Center, Added<Chunks>>
) {
  for center in &chunks {
    for lod in 1..res.ranges.len() - 1 {
      let keys = res.get_keys_by_lod(center.key, lod);
      request_load_chunk(&keys, &mut res, lod);
    }
  }
}

fn load_main_delta_octrees(
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

fn load_lod_delta_octrees(
  res: Res<BevyVoxelResource>,
  centers: Query<(&Center, &Chunks), Changed<Center>>
) {
  
  for (center, chunks) in &centers {
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
          let _ = res.send_process_mesh.send(data);
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
  res: Res<BevyVoxelResource>,
) {
  for c in res.recv_chunk.drain() {
    let _ = res.send_process_mesh.send(c.clone());
  }
}

fn receive_mesh(
  res: Res<BevyVoxelResource>,
  mut queries: Query<(&Center, &mut MeshComponent)>
) {
  for data in res.recv_mesh.drain() {
    for (center, mut mesh_comp) in &mut queries {
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


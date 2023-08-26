mod sphere;
mod cube;

use bevy::prelude::*;
use crate::{BevyVoxelResource, Selected, Preview, Chunks, Center, ChunkData, ShapeState, EditState};

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
      .add_system(added_chunks)
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

fn added_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks), Added<Chunks>>
) {
  for (center, mut chunks) in &mut chunks {
    let all_chunks = res.load_adj_mesh_data(center.key);
    chunks.data.clear();

    for (key, data) in all_chunks.iter() {
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: *key,
      });
    }

    let lod = res.chunk_manager.depth;

    let mut meshes = res.load_lod_meshes(center.key, lod as u8 - 1);
    meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 2));
    meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 3));
    for mesh in meshes.iter() {
      chunks.data.push(ChunkData {
        data: mesh.mesh.clone(),
        key: mesh.key,
      });
    }
  }
}

fn center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks), Changed<Center>>
) {
  for (center, mut chunks) in &mut centers {
    let all_chunks = res.load_adj_mesh_data(center.key);
    chunks.data.clear();

    for (key, data) in all_chunks.iter() {
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: *key,
      });
    }


    let lod = res.chunk_manager.depth;
    let mut meshes = res.load_lod_meshes(center.key, lod as u8 - 1);
    meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 2));
    meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 3));
    for mesh in meshes.iter() {
      chunks.data.push(ChunkData {
        data: mesh.mesh.clone(),
        key: mesh.key,
      });
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

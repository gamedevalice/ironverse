use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;
use crate::{EditState, BevyVoxelResource, Selected, Preview, Chunks, Center, ChunkData};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<EditState>()
      .insert_resource(BevyVoxelResource::default())
      .add_startup_system(startup)
      .add_system(update)
      .add_system(detect_selected_voxel_position)
      .add_system(detect_preview_voxel_position)
      .add_system(added_chunks)
      .add_system(center_changed);
  }
}

fn startup() {
  println!("startup BevyVoxel");
}

fn update(mut res: ResMut<BevyVoxelResource>) {
  res.physics.step();
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

fn detect_preview_voxel_position(
  mut cam: Query<(&Transform, &mut Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }
    let point = hit.unwrap();
    let pos = bevy_voxel_res.get_nearest_voxel_air(point);
    if pos.is_none() && preview.pos.is_some() {
      preview.pos = pos;
    }

    if pos.is_some() {
      if preview.pos.is_some() {
        let p = pos.unwrap();
        let current = preview.pos.unwrap();
        if current != p {
          preview.pos = pos;
        }
      }
      
      if preview.pos.is_none() {
        preview.pos = pos;
      }
    }
  }
}

fn added_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks), Added<Chunks>>
) {
  for (center, mut chunks) in &mut chunks {
    let all_chunks = res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }
}

fn center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks), Changed<Center>>
) {
  for (center, mut chunks) in &mut centers {
    let all_chunks = res.load_adj_chunks_with_collider(center.key);
    chunks.data.clear();

    for chunk in all_chunks.iter() {
      let data = res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }
}

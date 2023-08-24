use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;

use crate::{EditState, Preview, BevyVoxelResource, Center, Chunks, PreviewGraphics, ChunkData, ShapeState, Selected};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(preview_position.in_set(OnUpdate(EditState::RemoveNormal)))
      .add_system(add_voxel_cube.in_set(OnUpdate(EditState::RemoveNormal)))
      // .add_system(add_voxel_sphere.in_set(OnUpdate(EditState::RemoveNormal)))
      .add_system(remove.in_schedule(OnExit(EditState::RemoveNormal)))
      ;
  }
}

fn preview_position(
  mut previews: Query<(&Selected, &mut Preview), With<Preview>>,
) {
  for (selected, mut preview) in &mut previews {
    if preview.pos.is_none() && selected.pos.is_some() {
      preview.pos = selected.pos;
    }

    if preview.pos.is_some() && selected.pos.is_none() {
      preview.pos = selected.pos;
    }

    if preview.pos.is_some() && selected.pos.is_some() {
      let p = preview.pos.unwrap();
      let s = selected.pos.unwrap();
      if p != s {
        preview.pos = selected.pos;
      }
    }
  }
}


fn add_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Selected, &Center, &mut Chunks)>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Cube {
    return;
  }

  for (preview, selected, center, mut chunks) in &mut chunks {
    if selected.pos.is_none() {
      continue;
    }
    
    chunks.data.clear();
    let p = selected.pos.unwrap();

    // preview.voxel = 0;
    bevy_voxel_res.set_voxel_cube_default(p, preview.size, 0);
    // bevy_voxel_res.set_voxel(p, 0);

    let all_chunks = bevy_voxel_res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
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

fn add_voxel_sphere(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Sphere {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    chunks.data.clear();
    let p = preview.pos.unwrap();
    bevy_voxel_res.set_voxel_sphere(p, preview);

    let all_chunks = bevy_voxel_res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
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


fn remove(
  mut commands: Commands,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,
) {
  for entity in &preview_graphics {
    commands.entity(entity).despawn_recursive();
  }
}

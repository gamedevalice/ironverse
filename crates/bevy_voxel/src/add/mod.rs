use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;
use crate::{BevyVoxelResource, EditState, Chunks, Center, ChunkData, Preview, PreviewGraphics};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add_voxel.in_set(OnUpdate(EditState::AddNormal)))
      .add_system(remove.in_schedule(OnExit(EditState::AddNormal)));
  }
}

fn add_voxel(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
) {
  let mut voxel = None;
  
  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(1);
  }
  if voxel.is_none() {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    chunks.data.clear();
    let p = preview.pos.unwrap();
    bevy_voxel_res.set_voxel_by_preview(p, preview);

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


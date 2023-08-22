use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;
use crate::{BevyVoxelResource, EditState, Chunks, Center, ChunkData, Selected};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(remove_voxel.in_set(OnUpdate(EditState::RemoveNormal)));
  }
}


fn remove_voxel(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Selected, &Center, &mut Chunks)>,
) {
  let mut voxel = None;
  
  if mouse.just_pressed(MouseButton::Right) {
    voxel = Some(0);
  }
  if voxel.is_none() {
    return;
  }

  for (selected, center, mut chunks) in &mut chunks {
    println!("Remove voxel {:?}", selected.pos);
    if selected.pos.is_none() {
      continue;
    }

    
    chunks.data.clear();
    
    let p = selected.pos.unwrap();
    bevy_voxel_res.set_voxel(p, voxel.unwrap());

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


/*
  Manage all the data only
  Create cache later on if needed to
*/



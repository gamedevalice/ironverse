use bevy::prelude::*;
use crate::{BevyVoxelResource, EditState};


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

  // mut chunks: Query<(&Selected, &Player, &mut Chunks)>,
) {
  let mut voxel = None;
  if mouse.just_pressed(MouseButton::Right) {
    voxel = Some(0);
  }
  if voxel.is_none() {
    return;
  }

  // for (selected, player, mut chunks) in &mut chunks {
  //   if selected.pos.is_none() {
  //     continue;
  //   }
  //   for data in chunks.data.iter() {
  //     bevy_voxel_res.remove_collider(data.handle);
  //   }

  //   let p = selected.pos.unwrap();
  //   bevy_voxel_res.set_voxel(p, voxel.unwrap());

  //   let all_chunks = bevy_voxel_res.load_adj_chunks(player.key);
  //   for chunk in all_chunks.iter() {
  //     let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
  //     if data.positions.len() == 0 {
  //       continue;
  //     }

  //     let pos = bevy_voxel_res.get_pos(chunk.key);
      
  //     chunks.data.push(super::chunk::Mesh {
  //       key: chunk.key.clone(),
  //       data: data.clone(),
  //       chunk: chunk.clone(),
  //       handle: bevy_voxel_res.add_collider(pos, &data),
  //     });
  //   }
  // }
}


/*
  Manage all the data only
  Create cache later on if needed to
*/



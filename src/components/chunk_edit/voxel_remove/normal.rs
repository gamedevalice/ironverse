use bevy::prelude::*;
use voxels::chunk::chunk_manager::Chunk;
use crate::{data::GameResource, components::chunk_edit::{ChunkEditParams, ChunkEdit, get_point_by_edit_mode, EditState}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update_position.run_if(remove_normal))
      .add_system(
        position_changed.after(update_position).run_if(remove_normal)
      );
  }
}

fn remove_normal(state: Res<State<EditState>>) -> bool {
  state.0 == EditState::RemoveNormal
}

fn update_position(
  game_res: Res<GameResource>,
  mut chunk_edits: Query<(&Transform, &ChunkEditParams, &mut ChunkEdit)>,
) {
  for (trans, params, mut edit) in &mut chunk_edits {
    let min = 0;
    let max = params.size as i64;

    let mut pos_op = None;
    let total_div = 10;
    let min_dist = params.size as f32 * 2.0;

    'main: for i in (0..total_div).rev() {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (params.dist / div_f32) * i as f32;
      if dist < min_dist {
        break;
      }

      let p = get_point_by_edit_mode(&trans, dist, params.size, false);
      for x in min..max {
        for y in min..max {
          for z in min..max {
            let tmp_pos = [
              p.x as i64 + x,
              p.y as i64 + y,
              p.z as i64 + z
            ];
  
            let res = game_res.chunk_manager.get_voxel_safe(&tmp_pos);
            if res.is_some() && res.unwrap() == 0 {
              pos_op = Some(p);
              break 'main;
            }
          }
        }
      }
    }

    // println!("pos_op {:?}", pos_op);


    if pos_op.is_none() {
      if edit.position.is_some() {
        edit.position = None;
      }
      continue;
    }
    let pos = pos_op.unwrap();

    if edit.position.is_some() {
      let p = edit.position.unwrap();

      if p != pos {
        edit.position = Some(pos);
      }
    }

    if edit.position.is_none() {
      edit.position = Some(pos);
    }

    
  }
}

fn position_changed(
  mut edits: Query<(&mut ChunkEdit, &ChunkEditParams), Changed<ChunkEdit>>,
  game_res: Res<GameResource>,
) {
  for (mut edit, params) in &mut edits {
    if edit.position.is_none() {
      edit.chunk = None;
      continue;
    }
    
    let min = 0;
    let max = params.size as i64;

    let min_prev = min - 0;
    let max_prev = max + 0;
    let mut chunk = Chunk::default();
    let chunk_pos = game_res.chunk_manager.config.chunk_size / 2;
    for x in min_prev..max_prev {
      for y in min_prev..max_prev {
        for z in min_prev..max_prev {
          let local_x = chunk_pos as i64 + x;
          let local_y = chunk_pos as i64 + y;
          let local_z = chunk_pos as i64 + z;
          chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, 1);
        }
      }
    }

    edit.chunk = Some(chunk);
  }
}


use bevy::prelude::*;
use crate::data::GameResource;
use super::{ChunkEdit, get_snapped_position, ChunkEditResource, EditMode, get_point_by_edit_mode};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(on_change_voxel_changed)
      .add_system(update);
  }
}

fn on_change_voxel_changed(
  mut edits: Query<&mut ChunkEdit, With<DeleteNormal>>,
) {
  for mut edit in &mut edits {
    if edit.voxel != 0 {
      edit.voxel = 0;
    }
  }
}

fn update(
  mut chunk_edits: Query<(&Transform, &mut ChunkEdit), With<DeleteNormal>>,
  game_res: Res<GameResource>,
  chunk_edit_res: Res<ChunkEditResource>,
) {
  for (trans, mut edit) in &mut chunk_edits {
    let size = 2_u32.pow(edit.scale as u32);

    if edit.min != 0 {
      edit.min = 0;
    }
    if edit.max != size as i64 {
      edit.max = size as i64;
    }

    let mut pos_op = None;
    let total_div = 10;
    let max_dist = 12.0 + 12.0;
    'main: for i in 0..total_div {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (max_dist / div_f32) * i as f32;

      let mut snap = true;
      if chunk_edit_res.edit_mode == EditMode::DeleteNormal {
        snap = false;
      }

      let p = get_point_by_edit_mode(&trans, dist, size, snap);

      for x in edit.min..edit.max {
        for y in edit.min..edit.max {
          for z in edit.min..edit.max {
            let tmp_pos = [
              p.x as i64 + x,
              p.y as i64 + y,
              p.z as i64 + z
            ];
  
            let res = game_res.chunk_manager.get_voxel_safe(&tmp_pos);
            if res.is_some() && res.unwrap() == 1 {
              pos_op = Some(p);
              // info!("i {} dist {}", i, dist);
              break 'main;
            }
          }
        }
      }
    }

    if pos_op.is_none() {
      continue;
    }

    let pos = pos_op.unwrap();
    if edit.point_op.is_some() {
      let p = edit.point_op.unwrap();

      if p != pos {
        edit.point_op = Some(pos);
        edit.voxel = 0;
      }
    }

    if edit.point_op.is_none() {
      edit.point_op = Some(pos);
      edit.voxel = 0;
    }
  }
}


#[derive(Component)]
pub struct DeleteNormal {

}

impl Default for DeleteNormal {
  fn default() -> Self {
    Self {

    }
  }
}
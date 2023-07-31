use bevy::prelude::*;
use crate::data::GameResource;
use super::{ChunkEdit, get_snapped_position};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut chunk_edits: Query<(&Transform, &mut ChunkEdit), With<DeleteNormal>>,
  game_res: Res<GameResource>,
) {
  for (trans, mut edit) in chunk_edits.iter_mut() {
    let mut point = trans.translation + trans.forward() * edit.dist;
    let size = 2_u32.pow(edit.scale as u32);

    let min = 0;
    let max = size as i64;

    let mut result = None;

    let total_div = 10;
    let max_dist = 12.0 + 12.0;
    'main: for i in 0..total_div {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (max_dist / div_f32) * i as f32;

      let mut point = trans.translation + trans.forward() * dist;
      let size = 2_u32.pow(edit.scale as u32);
      point -= (size as f32 * 0.5 - 0.5);
      let p = get_snapped_position(point, size);

      // info!("range.dist {} dist {}", range.dist, dist);

      for x in min..max {
        for y in min..max {
          for z in min..max {
            let tmp_pos = [
              p.x as i64 + x,
              p.y as i64 + y,
              p.z as i64 + z
            ];
  
            let res = game_res.chunk_manager.get_voxel_safe(&tmp_pos);
            if res.is_some() && res.unwrap() == 1 {
              result = Some(p);
              // info!("i {} dist {}", i, dist);
              break 'main;
            }
          }
        }
      }
    }

    if result.is_none() {
      continue;
    }

    let pos = result.unwrap();

    let mut modified = false;
    if edit.point_op.is_some() {
      let p = edit.point_op.unwrap();

      if p != pos {
        edit.point_op = Some(pos);
        modified = true;
      }
    }

    if edit.point_op.is_none() {
      edit.point_op = Some(pos);
      modified = true;
    }












    // point -= (size as f32 * 0.5 - 0.5);
    // let pos = get_snapped_position(point, 1);

    // let mut modified = false;
    // if edit.point_op.is_some() {
    //   let p = edit.point_op.unwrap();

    //   if p != pos {
    //     edit.point_op = Some(pos);
    //     modified = true;
    //   }
    // }

    // if edit.point_op.is_none() {
    //   edit.point_op = Some(pos);
    //   modified = true;
    // }

    // if !modified { return; }

    // let size = 2_u32.pow(edit.scale as u32);
    // edit.min = 0;
    // edit.max = size as i64;
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
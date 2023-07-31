use bevy::prelude::*;
use crate::{components::chunk_edit::get_snapped_position, data::GameResource, input::hotbar::HotbarResource};
use super::ChunkEdit;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(on_change_voxel_changed)
      .add_system(update);
  }
}

fn on_change_voxel_changed(
  hotbar_res: Res<HotbarResource>,
  mut edits: Query<&mut ChunkEdit, With<CreateNormal>>,
) {

  let mut voxel_op = Some(1);
  for i in 0..hotbar_res.bars.len() {
    let bar = &hotbar_res.bars[i];
    if  hotbar_res.selected_keycode == bar.key_code {
      voxel_op = Some(bar.voxel);
    }

  }

  if voxel_op.is_none() { return; }

  for mut edit in &mut edits {
    if edit.voxel != voxel_op.unwrap() {
      edit.voxel = voxel_op.unwrap();
    }
  }
}

fn update(
  mut chunk_edits: Query<(&Transform, &mut ChunkEdit), With<CreateNormal>>,
  game_res: Res<GameResource>,
) {
  for (trans, mut edit) in chunk_edits.iter_mut() {
    let size = 2_u32.pow(edit.scale as u32);

    if edit.min != 0 {
      edit.min = 0;
    }
    if edit.max != size as i64 {
      edit.max = size as i64;
    }

    let mut pos_op = None;
    let total_div = 10;
    let min_dist = size as f32 * 2.0;
    'main: for i in (0..total_div).rev() {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (edit.dist / div_f32) * i as f32;
      if dist < min_dist {
        // info!("break");
        break;
      }

      let mut point = trans.translation + trans.forward() * dist;
      let size = 2_u32.pow(edit.scale as u32);
      point -= (size as f32 * 0.5 - 0.5);
      let p = get_snapped_position(point, 1);

      for x in edit.min..edit.max {
        for y in edit.min..edit.max {
          for z in edit.min..edit.max {
            let tmp_pos = [
              p.x as i64 + x,
              p.y as i64 + y,
              p.z as i64 + z
            ];
  
            let res = game_res.chunk_manager.get_voxel_safe(&tmp_pos);
            if res.is_some() && res.unwrap() == 0 {
              pos_op = Some(p);
              // info!("i {} dist {}", i, dist);
              break 'main;
            }
          }
        }
      }
    }


    if pos_op.is_none() {
      if edit.point_op.is_some() {
        edit.point_op = None;
      }
      continue;
    }
    let pos = pos_op.unwrap();

    if edit.point_op.is_some() {
      let p = edit.point_op.unwrap();

      if p != pos {
        edit.point_op = Some(pos);
      }
    }

    if edit.point_op.is_none() {
      edit.point_op = Some(pos);
    }
  }
}


#[derive(Component)]
pub struct CreateNormal {

}

impl Default for CreateNormal {
  fn default() -> Self {
    Self {

    }
  }
}






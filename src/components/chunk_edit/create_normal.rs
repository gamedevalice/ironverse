use bevy::{prelude::*, input::ButtonState};
use bevy_flycam::FlyCam;
use crate::input::{MouseInput, hotbar::HotbarResource};
use super::ChunkEdit;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut mouse_inputs: EventReader<MouseInput>,
  hotbar_res: Res<HotbarResource>,
  mut chunk_edits: Query<(&Transform, &mut ChunkEdit), With<FlyCam>>,
) {
  let mut voxel_op = None;

  // for event in mouse_inputs.iter() {
  //   if event.mouse_button_input.state == ButtonState::Pressed 
  //   && event.mouse_button_input.button == MouseButton::Left {
  //     voxel_op = Some(1);
  //     for i in 0..hotbar_res.bars.len() {
  //       let bar = &hotbar_res.bars[i];
  //       if  hotbar_res.selected_keycode ==  bar.key_code {
  //         voxel_op = Some(bar.voxel);
  //       }

  //     }
  //   }
  // }

  voxel_op = Some(1);
  for i in 0..hotbar_res.bars.len() {
    let bar = &hotbar_res.bars[i];
    if  hotbar_res.selected_keycode ==  bar.key_code {
      voxel_op = Some(bar.voxel);
    }

  }

  if voxel_op.is_none() {
    return;
  }

  // info!("Testing");

  for (trans, mut edit) in chunk_edits.iter_mut() {
    let mut point = trans.translation + trans.forward() * edit.dist;
    let size = 2_u32.pow(edit.scale as u32);
    point -= (size as f32 * 0.5 - 0.5);
    let pos = get_snapped_position(point, 1);

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

    if !modified { return; }

    info!("Point {:?}", point);

    let size = 2_u32.pow(edit.scale as u32);
    edit.min = 0;
    edit.max = size as i64;
    edit.voxel = voxel_op.unwrap();
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




fn get_snapped_position(pos: Vec3, size: u32) -> Vec3 {
  let adj_positions = get_nearby_snapped_positions(pos, size);

  let mut min_dist = f32::MAX;
  let mut snapped_pos = Vec3::ZERO;
  for adj_pos in adj_positions.iter() {
    let dist = pos.distance_squared(*adj_pos);

    if dist < min_dist {
      min_dist = dist;
      snapped_pos = *adj_pos;
    }
  }

  snapped_pos
}


fn get_nearby_snapped_positions(pos: Vec3, size: u32) -> Vec<Vec3> {
  let mut result = Vec::new();

  let size_i64 = size as i64;
  let base_x = ( (pos.x as i64) / size_i64 ) * size_i64;
  let base_y = ( (pos.y as i64) / size_i64 ) * size_i64;
  let base_z = ( (pos.z as i64) / size_i64 ) * size_i64;

  // println!("base_x {}", base_x);

  let range = 1;
  let min = -range;
  let max = range + 1;
  for x in min..max {
    for y in min..max {
      for z in min..max {
        let adj_x = base_x + (x * size_i64);
        let adj_y = base_y + (y * size_i64);
        let adj_z = base_z + (z * size_i64);

        result.push(Vec3::new(adj_x as f32, adj_y as f32, adj_z as f32));

        // println!("adj_x {}", adj_x);
      }
    }
  }
  

  result
}



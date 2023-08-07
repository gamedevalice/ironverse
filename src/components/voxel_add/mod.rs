use bevy::{prelude::*, input::mouse::MouseWheel};
use voxels::chunk::chunk_manager::Chunk;
use crate::{data::GameResource, components::get_point_by_edit_mode};
use super::{ChunkEdit, ChunkEditParams};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update_edit_params)
      .add_system(update_position)
      .add_system(position_changed.after(update_position));
  }
}

fn update_edit_params(
  mut mouse_wheels: EventReader<MouseWheel>,
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut chunk_edit_params: Query<&mut ChunkEditParams>,
) {
  for event in mouse_wheels.iter() {
    for mut params in chunk_edit_params.iter_mut() {
      // Need to clamp as event.y is returning -120.0 to 120.0 (Bevy bug)
      let seamless_size = 12 as f32;
      let adj = 12.0;
      let limit = seamless_size + adj;
      if params.dist <= limit {
        params.dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
      }
      
      if params.dist > limit {
        params.dist = limit;
      }

      let size = 2_u32.pow(params.level as u32);
      let min_val = size as f32;
      if params.dist < min_val {
        params.dist = min_val;
      }
    }
  }

  if key_input.just_pressed(KeyCode::Equals) {
    for mut params in chunk_edit_params.iter_mut() {
      if params.level < 3 {
        params.level += 1;
        params.size = 2_u32.pow(params.level as u32);
      }
    }
  }

  if key_input.just_pressed(KeyCode::Minus) {
    for mut params in chunk_edit_params.iter_mut() {
      if params.level > 0 {
        params.level -= 1;
        params.size = 2_u32.pow(params.level as u32);
      }
    }
  }
}

fn update_position(
  game_res: Res<GameResource>,
  mut chunk_edits: Query<(&Transform, &ChunkEditParams, &mut ChunkEdit)>,
  
) {
  for (trans, params, mut edit) in &mut chunk_edits {
    // println!("params {} {} {}", params.level, params.dist, params.size);

    let mut point = trans.translation + trans.forward() * params.dist;
    let min = 0;
    let max = params.size as i64;

    let mut pos_op = None;
    let total_div = 10;
    let min_dist = params.size as f32 * 2.0;

    // println!("point {:?}", point);
    // println!("size {}", params.size);
    'main: for i in (0..total_div).rev() {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (params.dist / div_f32) * i as f32;
      if dist < min_dist {
        break;
      }

      let p = get_point_by_edit_mode(&trans, dist, params.size, false);
      // println!("p {:?}", p);
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
              // info!("i {} dist {}", i, dist);
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
  mut game_res: ResMut<GameResource>,
) {
  for (mut edit, params) in &mut edits {
    if edit.position.is_none() {
      edit.chunk = None;
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();
    
    let mut min = 0;
    let mut max = params.size as i64;

    let point = edit.position.unwrap();

    let mut chunk = Chunk::default();
    let chunk_pos = game_res.chunk_manager.config.chunk_size / 2;

    for x in min..max {
      for y in min..max {
        for z in min..max {
          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];

          let _ = game_res.preview_chunk_manager.set_voxel2(&pos, 1);
        }
      }
    }

    let min_prev = min - 2;
    let max_prev = max + 2;
    for x in min_prev..max_prev {
      for y in min_prev..max_prev {
        for z in min_prev..max_prev {
          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];

          let v = game_res.preview_chunk_manager.get_voxel(&pos);

          let local_x = chunk_pos as i64 + x;
          let local_y = chunk_pos as i64 + y;
          let local_z = chunk_pos as i64 + z;
          chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, v);
        }
      }
    }

    // info!("chunk_edit_changed() {:?}", point);
    edit.chunk = Some(chunk);
  }
}






/*
  Define the components
    Preview
      Changeable based on selected voxel
      Add is different from remove
    Edit operation(Add)
      Different from remove
    Positioning
      Same with add and remove

  Treat everything as component
    To make it easier to modify
    Always treat the code as will be always be modified

  Main ingredients
    Selected voxel
    Defining the position
      Normal
      Snap to grid
    Size of chunk to edit
      Size of preview

  Data in/Data out
  Prefer top down approach than down to up when starting out the concept
    Prefer more control over encapsulation when implementing things
    Maximize control and transparency for now
      Be more conservative once it is established

  Centralized the data
  Show all the data
  Then treat behavior as a cartridge
  Then modularized it later on
*/















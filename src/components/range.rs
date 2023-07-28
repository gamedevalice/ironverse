use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_flycam::FlyCam;

use crate::data::GameResource;

use super::{player::Player, chunk_edit::{ChunkEdit, SnapMode, EditMode}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(init_resource)
      .add_system(add)
      .add_system(update_point.run_if(snap_normal))
      .add_system(update_point_snap_grid.run_if(not(snap_normal)))
      .add_system(update_range);
  }
}

fn init_resource(
  mut local_res: ResMut<LocalResource>,
  game_res: Res<GameResource>,
) {
  if local_res.max_dist == 0.0 {
    let adj = 12.0;
    local_res.max_dist = 
      game_res.chunk_manager.config.seamless_size as f32 + adj;
    println!("local_res {:?}", local_res.max_dist);
  }

  
}


fn add(
  mut commands: Commands,
  player_query: Query<Entity, Added<Player>>,
) {
  for entity in &player_query {
    commands
      .entity(entity)
      .insert(Range::default());
  }
}


fn snap_normal(
  query: Query<&ChunkEdit>,
) -> bool {
  for chunk_edit in query.iter() {
    if chunk_edit.snap_mode == SnapMode::Normal {
      return true;
    }
  }
  false
}


fn update_point(
  mut query: Query<(&Transform, &ChunkEdit, &mut Range), With<FlyCam>>,
) {
  for (trans, chunk_edit, mut range) in query.iter_mut() {
    let mut point = trans.translation + trans.forward() * range.dist;
    let size = 2_u32.pow(range.scale as u32);
    point += (size as f32 * 0.5);
    if range.point != point {
      range.point = point;
    }
  }
}

fn update_point_snap_grid(
  mut query: Query<(&Transform, &ChunkEdit, &mut Range), With<FlyCam>>,
  game_res: Res<GameResource>,
  local_res: Res<LocalResource>,
) {
  for (trans, chunk_edit, mut range) in query.iter_mut() {
    let default = false;

    if default {
      let mut point = trans.translation + trans.forward() * range.dist;    
      let size = 2_u32.pow(range.scale as u32);

      point -= (size as f32 * 0.5 - 0.5);
      let pos = get_snapped_position(point, size);
      if range.point != pos {
        range.point = pos;
      }
    } else {

      if chunk_edit.mode == EditMode::Create {
        let size = 2_u32.pow(range.scale as u32);
        let min = 0;
        let max = size as i64;

        let mut pos = Vec3::NAN;

        let total_div = 10;
        let min_dist = size as f32 * 2.0;
        'main: for i in (0..total_div).rev() {
          let div_f32 = total_div as f32 - 1.0;
          let dist = (range.dist / div_f32) * i as f32;
          if dist < min_dist {
            pos = Vec3::NAN;
            break;
          }

          let mut point = trans.translation + trans.forward() * dist;
          let size = 2_u32.pow(range.scale as u32);
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
                if res.is_some() && res.unwrap() == 0 {
                  pos = p;
                  // info!("i {} dist {}", i, dist);
                  break 'main;
                }
              }
            }
          }
        }
        
        if range.point.is_nan() ^ pos.is_nan() {
          range.point = pos;
        }

        if !range.point.is_nan() && !pos.is_nan() && range.point != pos {
          range.point = pos;
        }
      }


      if chunk_edit.mode == EditMode::Delete {
        let size = 2_u32.pow(range.scale as u32);
        let min = 0;
        let max = size as i64;

        let mut pos = Vec3::NAN;

        let total_div = 10;

        'main: for i in 0..total_div {
          let div_f32 = total_div as f32 - 1.0;
          let dist = (local_res.max_dist / div_f32) * i as f32;


          let mut point = trans.translation + trans.forward() * dist;
          let size = 2_u32.pow(range.scale as u32);
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
                  pos = p;
                  // info!("i {} dist {}", i, dist);
                  break 'main;
                }
              }
            }
          }
        }

        if range.point.is_nan() ^ pos.is_nan() {
          range.point = pos;
        }

        if !range.point.is_nan() && !pos.is_nan() && range.point != pos {
          range.point = pos;
        }
      }

      
    }
  }
}




fn update_range(
  mut query: Query<&mut Range>,
  mut mouse_wheels: EventReader<MouseWheel>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  for event in mouse_wheels.iter() {
    for mut range in query.iter_mut() {
      // Need to clamp as event.y is returning -120.0 to 120.0 (Bevy bug)
      let seamless_size = 12 as f32;
      let adj = 12.0;
      let limit = seamless_size + adj;
      if range.dist <= limit {
        range.dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
      }
      
      if range.dist > limit {
        range.dist = limit;
      }


      let size = 2_u32.pow(range.scale as u32);
      let min_val = size as f32;
      if range.dist < min_val {
        range.dist = min_val;
      }
      
    }
  }

  if keyboard_input.just_pressed(KeyCode::Equals) {
    for mut range in query.iter_mut() {
      if range.scale < 3 {
        range.scale += 1;
      }
    }
  }

  if keyboard_input.just_pressed(KeyCode::Minus) {
    for mut range in query.iter_mut() {
      if range.scale > 0 {
        range.scale -= 1;
        // info!("range.scale {}", range.scale);
      }
      
    }
  }
}

/*
  It is redundant, will rename/refactor it later
 */
#[derive(Component)]
pub struct Range {
  pub point: Vec3,
  pub dist: f32,
  pub scale: u8,
}

impl Default for Range {
  fn default() -> Self {
    Self {
      point: Vec3::new(f32::NAN, f32::NAN, f32::NAN),
      dist: 8.0,
      scale: 1,
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



#[derive(Resource, Default)]
struct LocalResource {
  pub max_dist: f32,
}


#[cfg(test)]
mod tests {
  use bevy::prelude::Vec3;
  use crate::components::range::get_snapped_position;
  use super::get_nearby_snapped_positions;


  #[test]
  fn test_get_all_nearby_snap_positive_positions1() {

    let size = 2;
    let pos = Vec3::new(2.1, 2.1, 2.1);
    let nearby_pos = get_nearby_snapped_positions(pos, size);

    let expected = [
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 2.0),
      Vec3::new(0.0, 0.0, 4.0),

      Vec3::new(0.0, 2.0, 0.0),
      Vec3::new(0.0, 2.0, 2.0),
      Vec3::new(0.0, 2.0, 4.0),

      Vec3::new(0.0, 4.0, 0.0),
      Vec3::new(0.0, 4.0, 2.0),
      Vec3::new(0.0, 4.0, 4.0),
      

      Vec3::new(2.0, 0.0, 0.0),
      Vec3::new(2.0, 0.0, 2.0),
      Vec3::new(2.0, 0.0, 4.0),

      Vec3::new(2.0, 2.0, 0.0),
      Vec3::new(2.0, 2.0, 2.0),
      Vec3::new(2.0, 2.0, 4.0),
     
      Vec3::new(2.0, 4.0, 0.0),
      Vec3::new(2.0, 4.0, 2.0),
      Vec3::new(2.0, 4.0, 4.0),
      
      
      Vec3::new(4.0, 0.0, 0.0),
      Vec3::new(4.0, 0.0, 2.0),
      Vec3::new(4.0, 0.0, 4.0),

      Vec3::new(4.0, 2.0, 0.0),
      Vec3::new(4.0, 2.0, 2.0),
      Vec3::new(4.0, 2.0, 4.0),

      Vec3::new(4.0, 4.0, 0.0),
      Vec3::new(4.0, 4.0, 2.0),
      Vec3::new(4.0, 4.0, 4.0),
    ];

    assert!(nearby_pos.len() > 0, "No result error");

    for pos in nearby_pos.iter() {
      assert!(expected.contains(pos), "{:?} should exists", pos);
    }
  }

  #[test]
  fn test_get_all_nearby_snap_positive_positions2() {

    let size = 2;
    let pos = Vec3::new(4.1, 4.1, 4.1);
    let nearby_pos = get_nearby_snapped_positions(pos, size);

    let expected = [
      Vec3::new(2.0, 2.0, 2.0),
      Vec3::new(2.0, 2.0, 4.0),
      Vec3::new(2.0, 2.0, 6.0),

      Vec3::new(2.0, 4.0, 2.0),
      Vec3::new(2.0, 4.0, 4.0),
      Vec3::new(2.0, 4.0, 6.0),
     
      Vec3::new(2.0, 6.0, 2.0),
      Vec3::new(2.0, 6.0, 4.0),
      Vec3::new(2.0, 6.0, 6.0),
      
      
      Vec3::new(4.0, 2.0, 2.0),
      Vec3::new(4.0, 2.0, 4.0),
      Vec3::new(4.0, 2.0, 6.0),

      Vec3::new(4.0, 4.0, 2.0),
      Vec3::new(4.0, 4.0, 4.0),
      Vec3::new(4.0, 4.0, 6.0),
     
      Vec3::new(4.0, 6.0, 2.0),
      Vec3::new(4.0, 6.0, 4.0),
      Vec3::new(4.0, 6.0, 6.0),

      Vec3::new(6.0, 2.0, 2.0),
      Vec3::new(6.0, 2.0, 4.0),
      Vec3::new(6.0, 2.0, 6.0),

      Vec3::new(6.0, 4.0, 2.0),
      Vec3::new(6.0, 4.0, 4.0),
      Vec3::new(6.0, 4.0, 6.0),
     
      Vec3::new(6.0, 6.0, 2.0),
      Vec3::new(6.0, 6.0, 4.0),
      Vec3::new(6.0, 6.0, 6.0),
    ];

    assert!(nearby_pos.len() > 0, "No result error");

    for pos in nearby_pos.iter() {
      assert!(expected.contains(pos), "{:?} should exists", pos);
    }
  }

  #[test]
  fn test_get_all_nearby_snap_negative_and_positive_positions() {

    let size = 2;
    let pos = Vec3::new(0.0, 0.0, 0.0);
    let nearby_pos = get_nearby_snapped_positions(pos, size);

    let expected = [
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-2.0, -2.0,  0.0),
      Vec3::new(-2.0, -2.0,  2.0),

      Vec3::new(-2.0,  0.0, -2.0),
      Vec3::new(-2.0,  0.0,  0.0),
      Vec3::new(-2.0,  0.0,  2.0),

      Vec3::new(-2.0,  2.0, -2.0),
      Vec3::new(-2.0,  2.0,  0.0),
      Vec3::new(-2.0,  2.0,  2.0),
      
      
      Vec3::new(0.0, -2.0, -2.0),
      Vec3::new(0.0, -2.0,  0.0),
      Vec3::new(0.0, -2.0,  2.0),

      Vec3::new(0.0,  0.0, -2.0),
      Vec3::new(0.0,  0.0,  0.0),
      Vec3::new(0.0,  0.0,  2.0),

      Vec3::new(0.0,  2.0, -2.0),
      Vec3::new(0.0,  2.0,  0.0),
      Vec3::new(0.0,  2.0,  2.0),


      Vec3::new(2.0, -2.0, -2.0),
      Vec3::new(2.0, -2.0,  0.0),
      Vec3::new(2.0, -2.0,  2.0),

      Vec3::new(2.0,  0.0, -2.0),
      Vec3::new(2.0,  0.0,  0.0),
      Vec3::new(2.0,  0.0,  2.0),

      Vec3::new(2.0,  2.0, -2.0),
      Vec3::new(2.0,  2.0,  0.0),
      Vec3::new(2.0,  2.0,  2.0),
    ];

    assert!(nearby_pos.len() > 0, "No result error");

    for pos in nearby_pos.iter() {
      assert!(expected.contains(pos), "{:?} should exists", pos);
    }
  }

  #[test]
  fn test_get_all_nearby_snap_negative_positions1() {

    let size = 2;
    let pos = Vec3::new(-2.1, -2.1, -2.1);
    let nearby_pos = get_nearby_snapped_positions(pos, size);

    let expected = [
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, -2.0),
      Vec3::new(0.0, 0.0, -4.0),

      Vec3::new(0.0, -2.0, 0.0),
      Vec3::new(0.0, -2.0, -2.0),
      Vec3::new(0.0, -2.0, -4.0),

      Vec3::new(0.0, -4.0, 0.0),
      Vec3::new(0.0, -4.0, -2.0),
      Vec3::new(0.0, -4.0, -4.0),
      

      Vec3::new(-2.0, 0.0, 0.0),
      Vec3::new(-2.0, 0.0, -2.0),
      Vec3::new(-2.0, 0.0, -4.0),

      Vec3::new(-2.0, -2.0, 0.0),
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-2.0, -2.0, -4.0),
     
      Vec3::new(-2.0, -4.0, 0.0),
      Vec3::new(-2.0, -4.0, -2.0),
      Vec3::new(-2.0, -4.0, -4.0),
      
      
      Vec3::new(-4.0, 0.0, 0.0),
      Vec3::new(-4.0, 0.0, -2.0),
      Vec3::new(-4.0, 0.0, -4.0),

      Vec3::new(-4.0, -2.0, 0.0),
      Vec3::new(-4.0, -2.0, -2.0),
      Vec3::new(-4.0, -2.0, -4.0),

      Vec3::new(-4.0, -4.0, 0.0),
      Vec3::new(-4.0, -4.0, -2.0),
      Vec3::new(-4.0, -4.0, -4.0),
    ];

    assert!(nearby_pos.len() > 0, "No result error");

    for pos in nearby_pos.iter() {
      assert!(expected.contains(pos), "{:?} should exists", pos);
    }
  }

  #[test]
  fn test_get_all_nearby_snap_negative_positions2() {

    let size = 2;
    let pos = Vec3::new(-4.1, -4.1, -4.1);
    let nearby_pos = get_nearby_snapped_positions(pos, size);

    let expected = [
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-2.0, -2.0, -4.0),
      Vec3::new(-2.0, -2.0, -6.0),

      Vec3::new(-2.0, -4.0, -2.0),
      Vec3::new(-2.0, -4.0, -4.0),
      Vec3::new(-2.0, -4.0, -6.0),
     
      Vec3::new(-2.0, -6.0, -2.0),
      Vec3::new(-2.0, -6.0, -4.0),
      Vec3::new(-2.0, -6.0, -6.0),
      
      
      Vec3::new(-4.0, -2.0, -2.0),
      Vec3::new(-4.0, -2.0, -4.0),
      Vec3::new(-4.0, -2.0, -6.0),

      Vec3::new(-4.0, -4.0, -2.0),
      Vec3::new(-4.0, -4.0, -4.0),
      Vec3::new(-4.0, -4.0, -6.0),
     
      Vec3::new(-4.0, -6.0, -2.0),
      Vec3::new(-4.0, -6.0, -4.0),
      Vec3::new(-4.0, -6.0, -6.0),

      Vec3::new(-6.0, -2.0, -2.0),
      Vec3::new(-6.0, -2.0, -4.0),
      Vec3::new(-6.0, -2.0, -6.0),

      Vec3::new(-6.0, -4.0, -2.0),
      Vec3::new(-6.0, -4.0, -4.0),
      Vec3::new(-6.0, -4.0, -6.0),
     
      Vec3::new(-6.0, -6.0, -2.0),
      Vec3::new(-6.0, -6.0, -4.0),
      Vec3::new(-6.0, -6.0, -6.0),
    ];

    assert!(nearby_pos.len() > 0, "No result error");

    for pos in nearby_pos.iter() {
      assert!(expected.contains(pos), "{:?} should exists", pos);
    }
  }



  #[test]
  fn test_snap_to_positive_positions() {
    let positions = [
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.9, 0.9, 0.9),
      Vec3::new(1.0, 1.0, 1.0),
      Vec3::new(1.1, 1.1, 1.1),
      Vec3::new(2.0, 2.0, 2.0),
      Vec3::new(3.0, 3.0, 3.0),
      Vec3::new(4.0, 4.0, 4.0),
    ];

    let expected = [
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(2.0, 2.0, 2.0),
      Vec3::new(2.0, 2.0, 2.0),
      Vec3::new(2.0, 2.0, 2.0),
      Vec3::new(4.0, 4.0, 4.0),
    ];

    let size = 2;
    for index in 0..positions.len() {
      let pos = &positions[index];
      let exp = &expected[index];
      let res = get_snapped_position(*pos, size);

      assert_eq!(
        res, *exp, "at index: {}, {:?} should be {:?}", index, res, exp
      );
    }
  }


  #[test]
  fn test_snap_to_negative_positions() {
    let positions = [
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(-0.9, -0.9, -0.9),
      Vec3::new(-1.0, -1.0, -1.0),
      Vec3::new(-1.1, -1.1, -1.1),
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-3.0, -3.0, -3.0),
      Vec3::new(-4.0, -4.0, -4.0),
    ];

    let expected = [
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-2.0, -2.0, -2.0),
      Vec3::new(-4.0, -4.0, -4.0),
      Vec3::new(-4.0, -4.0, -4.0),
    ];

    let size = 2;
    for index in 0..positions.len() {
      let pos = &positions[index];
      let exp = &expected[index];
      let res = get_snapped_position(*pos, size);
      
      assert_eq!(
        res, *exp, "at index: {}, {:?} should be {:?}", index, res, exp
      );
    }
  }






}
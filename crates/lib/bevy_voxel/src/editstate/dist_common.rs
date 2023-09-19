use bevy::{prelude::*, input::mouse::MouseWheel};
use utils::RayUtils;

use crate::{Preview, BevyVoxelResource, EditState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        (preview_position_by_dist, set_distance)
        .distributive_run_if(dist_state)
      );
  }
}

fn dist_state(edit_state: Res<State<EditState>>,) -> bool {
  edit_state.0 == EditState::AddDist ||
  edit_state.0 == EditState::RemoveDist
}

fn preview_position_by_dist(
  mut cam: Query<(&Transform, &mut Preview)>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let p = 
      cam_trans.translation + (cam_trans.forward() * preview.dist)
    ;

    let p1 = RayUtils::get_nearest_coord(
      [p.x, p.y, p.z], bevy_voxel_res.chunk_manager.voxel_scale
    );
    let pos = Some(Vec3::new(p1[0], p1[1], p1[2]));

    if pos.is_none() && preview.pos.is_some() {
      preview.pos = pos;
    }
    if pos.is_some() {
      if preview.pos.is_some() {
        let p = pos.unwrap();
        let current = preview.pos.unwrap();
        if current != p {
          preview.pos = pos;
        }
      }
      
      if preview.pos.is_none() {
        preview.pos = pos;
      }
    }
  }
}

fn set_distance(
  mut mouse_wheels: EventReader<MouseWheel>,
  time: Res<Time>,
  mut previews: Query<&mut Preview>,
) {
  for event in mouse_wheels.iter() {
    for mut params in previews.iter_mut() {
      // Need to clamp as event.y is returning -120.0 to 120.0 (Bevy bug)
      // let seamless_size = 12 as f32;
      // let adj = 12.0;
      // let max = seamless_size + adj;
      let max = 20.0;
      if params.dist <= max {
        params.dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 10.0;
      }
      
      if params.dist > max {
        params.dist = max;
      }

      // let size = 2_u32.pow(params.level as u32);
      // let min = size as f32;
      let min = 1.0;
      if params.dist < min {
        params.dist = min;
      }

      println!("dist {}", params.dist);
    }
  }
    
}

use bevy::{prelude::*, input::mouse::MouseWheel};
use utils::RayUtils;
use crate::{EditState, Preview, BevyVoxelResource, PreviewGraphics, ShapeState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, preview_position.run_if(in_state(EditState::RemoveSnap)))
      .add_systems(Update, set_distance.run_if(in_state(EditState::RemoveSnap)))
      .add_systems(Update, remove_voxel_cube.run_if(in_state(EditState::RemoveSnap)))
      .add_systems(Update, remove_voxel_sphere.run_if(in_state(EditState::RemoveSnap)))
      .add_systems(OnExit(EditState::RemoveSnap), remove)
      ;
  }
}

fn preview_position(
  mut cam: Query<(&Transform, &mut Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
  shape_state: Res<State<ShapeState>>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let p = cam_trans.translation + 
    (cam_trans.forward() * preview.dist);

    let mut snap_dist = bevy_voxel_res.chunk_manager.voxel_scale * preview.size as f32;
    if *State::get(&shape_state) == ShapeState::Sphere {

      let size = preview.sphere_size as i64;
      snap_dist = 
      bevy_voxel_res.chunk_manager.voxel_scale * 
      (size + size) as f32;
    }

    let tmp_p = RayUtils::get_nearest_coord(
      [p.x, p.y, p.z], snap_dist
    );
    let point = Vec3::new(tmp_p[0], tmp_p[1], tmp_p[2]);
    let pos = Some(point);

    // println!("point {:?} pos {:?}", point, pos);
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
    }
  }
    
}



fn remove_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut chunks: Query<&Preview>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  *State::get(&shape_state) != ShapeState::Cube {
    return;
  }

  for preview in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
  }
}


fn remove_voxel_sphere(
  mouse: Res<Input<MouseButton>>,
  mut chunks: Query<&Preview>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  *State::get(&shape_state) != ShapeState::Sphere {
    return;
  }

  for preview in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
  }
}

fn remove(
  mut commands: Commands,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,
) {
  for entity in &preview_graphics {
    commands.entity(entity).despawn_recursive();
  }
}

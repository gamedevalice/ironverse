use bevy::{prelude::*, input::mouse::MouseWheel};
use utils::RayUtils;
use voxels::data::voxel_octree::VoxelMode;
use crate::{EditState, Preview, BevyVoxelResource, Center, Chunks, PreviewGraphics, ChunkData, ShapeState};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(preview_position.in_set(OnUpdate(EditState::AddSnap)))
      .add_system(set_distance.in_set(OnUpdate(EditState::AddSnap)))
      .add_system(add_voxel_cube.in_set(OnUpdate(EditState::AddSnap)))
      .add_system(add_voxel_sphere.in_set(OnUpdate(EditState::AddSnap)))
      .add_system(remove.in_schedule(OnExit(EditState::AddSnap)))
      ;
  }
}

fn preview_position(
  mut cam: Query<(&Transform, &mut Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  shape_state: Res<State<ShapeState>>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let p = 
      cam_trans.translation + (cam_trans.forward() * preview.dist)
    ;

    let mut snap_dist = bevy_voxel_res.chunk_manager.voxel_scale * preview.size as f32;
    if shape_state.0 == ShapeState::Sphere {

      let size = preview.sphere_size as i64;
      snap_dist = 
      bevy_voxel_res.chunk_manager.voxel_scale * 
      (size + size) as f32;
    }

    let tmp_p = RayUtils::get_nearest_coord(
      // [p.x, p.y, p.z], bevy_voxel_res.chunk_manager.voxel_scale
      [p.x, p.y, p.z], snap_dist
    );
    let point = Vec3::new(tmp_p[0], tmp_p[1], tmp_p[2]);
    let pos = Some(point);


    // let pos = bevy_voxel_res.get_nearest_voxel_by_unit(point, preview.size as f32);

    // println!("point {:?} pos {:?}, size {:?}", point, pos, preview.size);
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



fn add_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Cube {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    chunks.data.clear();
    let p = preview.pos.unwrap();
    bevy_voxel_res.set_voxel_cube(p, preview);

    let all_chunks = bevy_voxel_res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }
}

fn add_voxel_sphere(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Sphere {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    chunks.data.clear();
    let p = preview.pos.unwrap();
    bevy_voxel_res.set_voxel_sphere(p, preview);

    let all_chunks = bevy_voxel_res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
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

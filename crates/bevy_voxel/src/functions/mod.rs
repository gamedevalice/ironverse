mod sphere;
mod cube;

use bevy::{prelude::*, input::mouse::MouseWheel};
use voxels::data::voxel_octree::VoxelMode;
use crate::{BevyVoxelResource, Selected, Preview, Chunks, Center, ChunkData, ShapeState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(sphere::CustomPlugin)
      .add_plugin(cube::CustomPlugin)
      .insert_resource(BevyVoxelResource::default())
      .add_startup_system(startup)
      .add_system(update)
      .add_system(detect_selected_voxel_position)
      .add_system(detect_preview_voxel_position)
      .add_system(added_chunks)
      .add_system(center_changed)
      .add_system(shape_state_changed)
      // .add_system(preview_params)
      ;
  }
}

fn startup() {
  println!("startup BevyVoxel");
}

fn update(
  mut res: ResMut<BevyVoxelResource>,
  shape_state: Res<State<ShapeState>>,
) {
  res.physics.step();
  res.shape_state = shape_state.0;
}

fn detect_selected_voxel_position(
  mut cam: Query<(&Transform, &mut Selected), With<Selected>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut selected) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }

    let pos = bevy_voxel_res.get_hit_voxel_pos(hit.unwrap());
    if pos.is_none() && selected.pos.is_some() {
      selected.pos = pos;
    }

    if pos.is_some() {
      if selected.pos.is_some() {
        let p = pos.unwrap();
        let current = selected.pos.unwrap();
        if current != p {
          selected.pos = pos;
        }
      }
      
      if selected.pos.is_none() {
        selected.pos = pos;
      }
    }
  }
}

fn detect_preview_voxel_position(
  mut cam: Query<(&Transform, &mut Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }
    let point = hit.unwrap();
    let pos = bevy_voxel_res.get_nearest_voxel_air(point);
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

fn added_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks), Added<Chunks>>
) {
  for (center, mut chunks) in &mut chunks {
    let all_chunks = res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = res.compute_mesh(VoxelMode::SurfaceNets, chunk);
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

fn center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks), Changed<Center>>
) {
  for (center, mut chunks) in &mut centers {
    let all_chunks = res.load_adj_chunks_with_collider(center.key);
    chunks.data.clear();

    for chunk in all_chunks.iter() {
      let data = res.compute_mesh(VoxelMode::SurfaceNets, chunk);
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


fn shape_state_changed(
  shape_state: Res<State<ShapeState>>,
  mut local: Local<ShapeState>,
  mut previews: Query<&mut Preview>,
) {
  if *local != shape_state.0 {
    *local = shape_state.0;
    for mut preview in &mut previews {
      preview.size = preview.size;
    }
  }
  
}

fn preview_params(
  mut mouse_wheels: EventReader<MouseWheel>,
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut previews: Query<&mut Preview>,
) {
  for event in mouse_wheels.iter() {
    // for mut params in previews.iter_mut() {
    //   // Need to clamp as event.y is returning -120.0 to 120.0 (Bevy bug)
    //   let seamless_size = 12 as f32;
    //   let adj = 12.0;
    //   let limit = seamless_size + adj;
    //   if params.dist <= limit {
    //     params.dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
    //   }
      
    //   if params.dist > limit {
    //     params.dist = limit;
    //   }

    //   let size = 2_u32.pow(params.level as u32);
    //   let min_val = size as f32;
    //   if params.dist < min_val {
    //     params.dist = min_val;
    //   }
    // }
  }

  // if key_input.just_pressed(KeyCode::Equals) {
  //   for mut preview in previews.iter_mut() {
  //     if preview.level < 3 {
  //       preview.level += 1;
  //       preview.size = 2_u8.pow(preview.level as u32);
  //     }
  //   }
  // }

  // if key_input.just_pressed(KeyCode::Minus) {
  //   for mut preview in previews.iter_mut() {
  //     if preview.level > 0 {
  //       preview.level -= 1;
  //       preview.size = 2_u8.pow(preview.level as u32);
  //     }
  //   }
  // }

  let speed = 5.0;
  if key_input.pressed(KeyCode::Equals) {
    for mut preview in previews.iter_mut() {
      if preview.sphere_size < 8.0 {
        preview.sphere_size += time.delta_seconds() * speed;
      }
    }
  }

  if key_input.pressed(KeyCode::Minus) {
    for mut preview in previews.iter_mut() {
      if preview.sphere_size > 1.0 {
        preview.sphere_size -= time.delta_seconds() * speed;
      }
    }
  }
    
}


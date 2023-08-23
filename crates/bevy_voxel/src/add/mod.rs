use bevy::{prelude::*, input::mouse::MouseWheel};
use voxels::data::voxel_octree::VoxelMode;
use crate::{BevyVoxelResource, EditState, Chunks, Center, ChunkData, Preview, PreviewGraphics};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add_voxel.in_set(OnUpdate(EditState::AddNormal)))
      .add_system(remove.in_schedule(OnExit(EditState::AddNormal)))
      .add_system(preview_params.in_set(OnUpdate(EditState::AddDist)));
  }
}

fn add_voxel(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
) {
  let mut voxel = None;
  
  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(1);
  }
  if voxel.is_none() {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    chunks.data.clear();
    let p = preview.pos.unwrap();
    bevy_voxel_res.set_voxel_by_preview(p, preview);

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

fn preview_params(
  mut mouse_wheels: EventReader<MouseWheel>,
  key_input: Res<Input<KeyCode>>,
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
        params.dist += max.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
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

use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;
use crate::{EditState, Preview, BevyVoxelResource, Center, Chunks, PreviewGraphics, ChunkData, ShapeState, MeshComponent};

use super::{EditEvents, EditEvent};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add_voxel_cube.in_set(OnUpdate(EditState::AddNormal)))
      .add_system(add_voxel_sphere.in_set(OnUpdate(EditState::AddNormal)))
      .add_system(remove.in_schedule(OnExit(EditState::AddNormal)));
  }
}

fn add_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut chunks: Query<&Preview>,
  shape_state: Res<State<ShapeState>>,
  
  mut edit_event_writer: EventWriter<EditEvents>
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Cube {
    return;
  }

  for preview in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
    edit_event_writer.send(EditEvents {
      event: EditEvent::AddCube
    });
  }
}

fn add_voxel_sphere(
  mouse: Res<Input<MouseButton>>,
  mut chunks: Query<&Preview>,
  shape_state: Res<State<ShapeState>>,
  mut edit_event_writer: EventWriter<EditEvents>
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Sphere {
    return;
  }

  for preview in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
    edit_event_writer.send(EditEvents {
      event: EditEvent::AddSphere
    });
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

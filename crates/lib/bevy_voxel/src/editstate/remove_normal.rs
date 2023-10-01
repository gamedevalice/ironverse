use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;

use crate::{EditState, Preview, BevyVoxelResource, Center, Chunks, PreviewGraphics, ChunkData, ShapeState, Selected};

use super::{EditEvents, EditEvent};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, preview_position.run_if(in_state(EditState::RemoveNormal)))
      .add_systems(Update, remove_voxel_cube.run_if(in_state(EditState::RemoveNormal)))
      .add_systems(Update, remove_voxel_sphere.run_if(in_state(EditState::RemoveNormal)))
      .add_systems(OnExit(EditState::RemoveNormal), remove)
      ;
  }
}

fn preview_position(
  mut previews: Query<(&Selected, &mut Preview), With<Preview>>,
) {
  for (selected, mut preview) in &mut previews {
    if preview.pos.is_none() && selected.pos.is_some() {
      preview.pos = selected.pos;
    }

    if preview.pos.is_some() && selected.pos.is_none() {
      preview.pos = selected.pos;
    }

    if preview.pos.is_some() && selected.pos.is_some() {
      let p = preview.pos.unwrap();
      let s = selected.pos.unwrap();
      if p != s {
        preview.pos = selected.pos;
      }
    }
  }
}


fn remove_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<&Selected>,
  shape_state: Res<State<ShapeState>>,
  mut edit_event_writer: EventWriter<EditEvents>
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  *State::get(&shape_state) != ShapeState::Cube {
    return;
  }

  for selected in &mut chunks {
    if selected.pos.is_none() {
      continue;
    }
    // edit_event_writer.send(EditEvents {
    //   event: EditEvent::RemoveCube
    // });
  }
}

fn remove_voxel_sphere(
  mouse: Res<Input<MouseButton>>,
  mut chunks: Query<&Preview>,
  shape_state: Res<State<ShapeState>>,
  mut edit_event_writer: EventWriter<EditEvents>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  *State::get(&shape_state) != ShapeState::Sphere {
    return;
  }

  for preview in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
    // edit_event_writer.send(EditEvents {
    //   event: EditEvent::RemoveSphere
    // });
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

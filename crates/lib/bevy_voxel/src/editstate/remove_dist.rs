use bevy::prelude::*;
use crate::{EditState, Preview, ShapeState, PreviewGraphics, Selected};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, remove_voxel_cube.run_if(in_state(EditState::RemoveDist)))
      .add_systems(Update, remove_voxel_sphere.run_if(in_state(EditState::RemoveDist)))
      .add_systems(OnExit(EditState::RemoveDist), remove);
  }
}

fn remove_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut chunks: Query<&Selected>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  *State::get(&shape_state) != ShapeState::Cube {
    return;
  }

  for selected in &mut chunks {
    if selected.pos.is_none() {
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

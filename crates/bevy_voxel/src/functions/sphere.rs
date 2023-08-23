use bevy::prelude::*;
use crate::{Preview, ShapeState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
    .add_system(modify_preview.in_set(OnUpdate(ShapeState::Sphere)));
  }
}

fn modify_preview(
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut previews: Query<&mut Preview>,
) {
  let speed = 1.0;
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
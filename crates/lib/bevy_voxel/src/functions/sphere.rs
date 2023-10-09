use bevy::prelude::*;
use crate::{Preview, ShapeState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
    .add_systems(Update, modify_preview.run_if(in_state(ShapeState::Sphere)));
  }
}

fn modify_preview(
  _key_input: Res<Input<KeyCode>>,
  _time: Res<Time>,
  mut _previews: Query<&mut Preview>,
) {
  let _speed = 1.0;
  // if key_input.pressed(KeyCode::Equals) {
  //   for mut preview in previews.iter_mut() {
  //     if preview.sphere_size < 8.0 {
  //       preview.sphere_size += time.delta_seconds() * speed;

  //       println!("sphere_size {}", preview.sphere_size);
  //     }
  //   }
  // }

  // if key_input.pressed(KeyCode::Minus) {
  //   for mut preview in previews.iter_mut() {
  //     if preview.sphere_size > 1.0 {
  //       preview.sphere_size -= time.delta_seconds() * speed;

  //       println!("sphere_size {}", preview.sphere_size);
  //     }
  //   }
  // }
}
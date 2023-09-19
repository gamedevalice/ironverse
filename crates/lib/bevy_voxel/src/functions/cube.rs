use bevy::prelude::*;
use crate::{Preview, ShapeState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
    .add_system(modify_preview.in_set(OnUpdate(ShapeState::Cube)));
  }
}

fn modify_preview(
  key_input: Res<Input<KeyCode>>,
  mut previews: Query<&mut Preview>,
) {
  if key_input.just_pressed(KeyCode::Equals) {
    for mut preview in previews.iter_mut() {
      if preview.level < 3 {
        preview.level += 1;
        preview.size = 2_u8.pow(preview.level as u32);
      }
    }
  }

  if key_input.just_pressed(KeyCode::Minus) {
    for mut preview in previews.iter_mut() {
      if preview.level > 0 {
        preview.level -= 1;
        preview.size = 2_u8.pow(preview.level as u32);
      }
    }
  }

}
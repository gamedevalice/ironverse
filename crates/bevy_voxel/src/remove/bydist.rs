use bevy::prelude::*;

use crate::{EditState, Preview};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(enter.in_schedule(OnEnter(EditState::RemoveDist)));
  }
}

fn enter(
  mut previews: Query<&mut Preview>,
) {

  println!("enter() RemoveDist");
  for mut preview in &mut previews {
    preview.size = preview.size;
  }
} 
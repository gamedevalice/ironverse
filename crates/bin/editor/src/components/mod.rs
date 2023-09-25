use bevy::prelude::*;

pub mod camera;
pub mod player;

pub mod chunk_edit;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(chunk_edit::CustomPlugin);
  }
}
use bevy::prelude::*;

pub mod chunks;
pub mod chunk_preview;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(chunks::CustomPlugin)
      // .add_plugin(chunk_preview::CustomPlugin)
      ;
  }
}
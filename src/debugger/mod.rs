use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}};

// pub mod raycast;
// mod camera;
mod text;
// mod chunks;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(text::CustomPlugin)
      // .add_plugin(LogDiagnosticsPlugin::default())
      // .add_plugin(raycast::CustomPlugin)
      // .add_plugin(camera::CustomPlugin)
      // .add_plugin(chunks::CustomPlugin)
      ;
  }
}
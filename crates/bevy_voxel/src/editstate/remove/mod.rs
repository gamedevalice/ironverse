use bevy::prelude::*;

mod normal;
mod bydist;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(normal::CustomPlugin)
      .add_plugin(bydist::CustomPlugin);
  }
}
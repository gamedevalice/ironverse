use bevy::prelude::*;

mod add_normal;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(add_normal::CustomPlugin);
  }
}
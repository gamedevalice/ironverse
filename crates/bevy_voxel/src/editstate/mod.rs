use bevy::prelude::*;

mod add_normal;
mod add_dist;

mod dist_common;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(add_normal::CustomPlugin)
      .add_plugin(add_dist::CustomPlugin)
      .add_plugin(dist_common::CustomPlugin);
  }
}
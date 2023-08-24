use bevy::prelude::*;

mod add_normal;
mod add_dist;
mod add_snap;

mod remove_normal;
mod remove_dist;

mod dist_common;
mod normal_common;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(add_normal::CustomPlugin)
      .add_plugin(add_dist::CustomPlugin)
      .add_plugin(add_snap::CustomPlugin)
      
      .add_plugin(remove_normal::CustomPlugin)
      .add_plugin(remove_dist::CustomPlugin)
      .add_plugin(normal_common::CustomPlugin)
      .add_plugin(dist_common::CustomPlugin);
  }
}
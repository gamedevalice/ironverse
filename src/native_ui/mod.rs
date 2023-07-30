mod save;
mod load;

use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(save::CustomPlugin)
      .add_plugin(load::CustomPlugin);
  }
}

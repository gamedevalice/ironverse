mod save;
mod load;

use bevy::prelude::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(save::CustomPlugin)
      .add_plugins(load::CustomPlugin);
  }
}

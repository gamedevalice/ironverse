use bevy::prelude::*;
use crate::components::player::Player;

mod normal;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(normal::CustomPlugin)
      .add_system(add);
  }
}

fn add(
  mut commands: Commands,
  player_query: Query<Entity, Added<Player>>,
) {
  for entity in &player_query {
    commands
      .entity(entity)
      // .insert(Normal::default())
      ;
  }
}




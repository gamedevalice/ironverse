use bevy::prelude::*;
use crate::{data::{GameResource, GameState, Data, UIState}, physics::Physics, components::player::Player, graphics::ChunkGraphics};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::New), enter);
  }
}

fn enter(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,
  player_query: Query<(Entity, &Player)>,
  mut game_state_next: ResMut<NextState<GameState>>,
  mut ui_state_next: ResMut<NextState<UIState>>,

  chunk_graphics: Query<Entity, With<ChunkGraphics>>,
) {
  game_res.chunk_manager.chunks.clear();
  *physics = Physics::default();
  
  for (entity, _) in &player_query {
    commands.entity(entity).despawn_recursive();
  }
  
  for entity in &chunk_graphics {
    commands.entity(entity).despawn_recursive();
  }

  game_res.data = Data::default();

  ui_state_next.set(UIState::Default);

  game_state_next.set(GameState::Init);

  info!("new dispose");
}

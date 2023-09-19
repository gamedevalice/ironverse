use bevy::prelude::*;
use crate::data::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        enter.in_schedule(OnEnter(GameState::Init))
      )
      // .add_system(update)
      ;
  }
}

fn enter(
  mut game_state_next: ResMut<NextState<GameState>>,
) {
  game_state_next.set(GameState::Play);
}

fn update(
  mut light_query: Query<&mut Transform, With<PointLight>>,
  time: Res<Time>,
) {
  let t = time.elapsed_seconds();
  for mut tfm in light_query.iter_mut() {
    tfm.translation = 5.0 * Vec3::new(t.cos(), 1.0, t.sin());
  }
}


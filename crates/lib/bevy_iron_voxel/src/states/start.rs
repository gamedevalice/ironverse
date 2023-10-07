use bevy::prelude::*;
use crate::data::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Start), 
        enter
      )
      .add_systems(Update, update)
      ;
  }
}

fn enter(
  mut game_state_next: ResMut<NextState<GameState>>,
) {
  game_state_next.set(GameState::Play);
}

fn update(
  mut _light_query: Query<&mut Transform, With<PointLight>>,
  _time: Res<Time>,
) {
  // let t = time.elapsed_seconds();
  // for mut tfm in light_query.iter_mut() {
  //   tfm.translation = 5.0 * Vec3::new(t.cos(), 1.0, t.sin());
  // }
}


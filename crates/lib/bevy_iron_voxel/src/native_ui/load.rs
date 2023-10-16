use bevy::prelude::*;
use std::fs;
use crate::data::{GameState, Data, GameResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::LoadGame), enter)
      .add_systems(Update, update.run_if(in_state(GameState::LoadGame)))
      .add_systems(OnExit(GameState::LoadGame), exit)
      ;
  }
}

fn enter(
  mut game_res: ResMut<GameResource>,
  mut game_state_next: ResMut<NextState<GameState>>,
) {
  if let Some(path) = rfd::FileDialog::new().pick_file() {
    let contents = match fs::read_to_string(path.clone()) {
      Ok(c) => c,
      Err(_) => {
        info!("Could not read file `{:?}`", path);
        "".to_string()
      }
    };

    let data: Data = match toml::from_str(&contents) {
      Ok(d) => d,
      Err(_) => Data::default()
    };
    game_res.data = data;
    game_state_next.set(GameState::Load);
  }
}

fn update() {

}

fn exit() {
  // info!("exit");
}
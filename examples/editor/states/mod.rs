use bevy::prelude::*;

mod start;
mod new;
mod load;


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(start::CustomPlugin)
      .add_plugin(new::CustomPlugin)
      .add_plugin(load::CustomPlugin);
  }
}

pub struct GameEvent {
  pub event_type: GameEventType,
  pub pos: Vec3,
}

impl GameEvent {
  pub fn new(e: GameEventType, p: Vec3) -> Self {
    Self {
      event_type: e,
      pos: p
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum GameEventType {
  SpawnPlayer
}



/*
  Spawn player
  Spawn terrains around

 */

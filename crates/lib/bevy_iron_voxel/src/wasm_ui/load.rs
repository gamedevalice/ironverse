use bevy::prelude::*;
use flume::{Sender, Receiver};
use serde::{Deserialize, Serialize};
use crate::data::{Data, GameResource, GameState};
use std::future::Future;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_systems(OnEnter(GameState::LoadGame), enter)
      .add_systems(Update, update.run_if(in_state(GameState::LoadGame)))
      .add_systems(OnExit(GameState::LoadGame), exit)
      ;
  }
}

fn enter(
  mut game_res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,
) {
  load_file(local_res.send.clone());
}

fn update(
  local_res: Res<LocalResource>,
  mut game_res: ResMut<GameResource>,
  mut game_state_next: ResMut<NextState<GameState>>,
) {
  for file in local_res.recv.drain() {
    let s = match String::from_utf8(file) {
      Ok(v) => v,
      Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let data: Data = toml::from_str(&s).unwrap();
    game_res.data = data;
    game_state_next.set(GameState::Load);
  }
}

fn exit() {
  // info!("exit");
}


fn load_file(send: Sender<Vec<u8>>) {
  let task = rfd::AsyncFileDialog::new().pick_file();

  let send = send.clone();
  execute(async move {
    let file = task.await;
    if let Some(file) = file {
      let res = file.read().await;
      send.send(res);
    }
  });
}


fn execute<F: Future<Output = ()> + 'static>(f: F) {
  wasm_bindgen_futures::spawn_local(f);
}



#[derive(Resource)]
struct LocalResource {
  send: Sender<Vec<u8>>,
  recv: Receiver<Vec<u8>>,
}

impl Default for LocalResource {
  fn default() -> Self {
    let (send, recv) = flume::bounded(1);
    Self {
      send: send,
      recv: recv,
    }
  }
}

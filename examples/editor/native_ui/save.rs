use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::Task;
// use crate::components::chunk::Chunks;
use crate::components::player::Player;
use crate::data::CursorState;
use crate::data::Data;
use crate::data::GameResource;
use crate::data::Status;
use crate::data::Terrains;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::data::GameState;
use futures_lite::future;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_system(track_modified_chunks)
      .add_system(enter.in_schedule(OnEnter(GameState::SaveGame)))
      .add_system(export_obj)
      .add_system(handle_obj_export);
  }
}

fn enter(
  game_res: Res<GameResource>,
  players: Query<&Transform, With<Player>>,
) {
  let mut terrains = Terrains { keys: Vec::new(), voxels: Vec::new() };
  for (key, chunk) in game_res.modified_chunks.iter() {
    terrains.keys.push(key.clone());
    terrains.voxels.push(array_bytes::bytes2hex("", &chunk.octree.data));
  }

  let mut pos = Vec3::ZERO;
  for trans in &players {
    pos = trans.translation;
  }

  let data = Data {
    status: Status {
      position: pos.into(),
    },
    terrains: terrains
  };

  let str = toml::to_string_pretty(&data).unwrap();
  let path = std::env::current_dir().unwrap();
  let res = rfd::FileDialog::new()
    .set_file_name("save.toml")
    .set_directory(&path)
    .save_file();

  if res.is_none() {
    return;
  }

  let p = res.unwrap();
  let mut data_file = File::create(p).expect("creation failed");
  data_file.write(str.as_bytes()).expect("write failed");
}

/* 
fn track_modified_chunks(
  mut chunks_query: Query<&Chunks, Changed<Chunks>>,
  mut game_res: ResMut<GameResource>,
) {
  for c in &chunks_query {
    for mesh in c.data.iter() {
      if !mesh.chunk.is_default {
        game_res.modified_chunks.insert(mesh.key.clone(), mesh.chunk.clone());
        // info!("mesh.key {:?}", mesh.key);
      }
    }
    
  }
}

 */
fn export_obj(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,

  mut cursor_state_next: ResMut<NextState<CursorState>>,
) {
  if game_res.export_obj.is_some() {
    cursor_state_next.set(CursorState::None);

    let str = game_res.export_obj.take().unwrap();
    let main_dir = std::env::current_dir().unwrap();
    let main_str = main_dir.to_str().unwrap();
    let path = format!("{}/assets", main_str);

    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
      let res = rfd::FileDialog::new()
        .set_file_name("save.obj")
        .set_directory(&path)
        .save_file();

      Result { path: res, str: str }
    });

    commands.spawn(FileTask(task));
  }
}


fn handle_obj_export(
  mut commands: Commands,
  mut transform_tasks: Query<(Entity, &mut FileTask)>,

  mut cursor_state_next: ResMut<NextState<CursorState>>,
) {
  for (entity, mut task) in &mut transform_tasks {
    if let Some(res) = future::block_on(future::poll_once(&mut task.0)) {
      cursor_state_next.set(CursorState::Locked);

      if res.path.is_some() {
        let p = res.path.unwrap();
        let mut data_file = File::create(p).expect("creation failed");
        data_file.write(res.str.as_bytes()).expect("write failed");
      }
      
      commands.entity(entity).remove::<FileTask>();
    }
  }
}



#[derive(Component)]
struct FileTask(Task<Result>);

struct Result {
  path: Option<PathBuf>,
  str: String,
}


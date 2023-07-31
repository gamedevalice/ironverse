use bevy::prelude::*;
use voxels::chunk::{chunk_manager::Chunk};
use crate::{data::{GameResource}, utils::{nearest_voxel_point_0, nearest_voxel_point}, input::hotbar::{HotbarResource, self}};
use super::{raycast::Raycast, range::Range, player::Player, chunk_edit::ChunkEdit};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(add)
      .add_system(chunk_edit_changed)
      .add_system(on_changed_selected_voxel)
      .add_system(on_remove);
  }
}

fn add(
  mut commands: Commands,
  players: Query<(Entity), Added<Player>>,
) {
  for entity in &players {
    commands
      .entity(entity)
      .insert(ChunkPreview::default());
  }
}

fn chunk_edit_changed(
  mut game_res: ResMut<GameResource>,
  mut previews: Query<
    (&ChunkEdit, &mut ChunkPreview), Changed<ChunkEdit>
  >,
  hotbar_res: Res<HotbarResource>,
) {
  for (edit, mut chunk_preview) in &mut previews {
    if edit.point_op.is_none() {
      chunk_preview.chunk_op = None;
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();
    
    let mut min = edit.min;
    let mut max = edit.max;

    let point = edit.point_op.unwrap();

    chunk_preview.new = [
      point.x as i64,
      point.y as i64,
      point.z as i64,
    ];

    let mut chunk = Chunk::default();
    let chunk_pos = game_res.chunk_manager.config.chunk_size / 2;

    let mut voxel = edit.voxel;
    if voxel == 0 {
      voxel = 1;
    }
    for x in min..max {
      for y in min..max {
        for z in min..max {
          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];

          let _ = game_res.preview_chunk_manager.set_voxel2(&pos, voxel);
        }
      }
    }

    let min_prev = min - 2;
    let max_prev = max + 2;
    for x in min_prev..max_prev {
      for y in min_prev..max_prev {
        for z in min_prev..max_prev {
          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];

          let v = game_res.preview_chunk_manager.get_voxel(&pos);

          let local_x = chunk_pos as i64 + x;
          let local_y = chunk_pos as i64 + y;
          let local_z = chunk_pos as i64 + z;
          chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, v);
        }
      }
    }

    info!("chunk_edit_changed() {:?}", point);
    chunk_preview.chunk_op = Some(chunk);
  }
}

fn on_changed_selected_voxel(
  mut game_res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,
  hotbar_res: Res<HotbarResource>,

  mut ranges: Query<(&Range, &mut ChunkPreview)>,
) {
  if local_res.selected_keycode == hotbar_res.selected_keycode {
    return;
  }
  local_res.selected_keycode = hotbar_res.selected_keycode;

  let bar_op = hotbar_res
    .bars
    .iter()
    .find(|bar| bar.key_code == hotbar_res.selected_keycode);

  let mut voxel = 0;
  if bar_op.is_some() {
    voxel = bar_op.unwrap().voxel;
  }

  for (range, mut chunk_preview) in &mut ranges {
    if range.point.x == f32::NAN {
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();

    let min = -(range.scale as i64);
    let max = (range.scale as i64) + 1;

    // info!("min {} max {}", min, max);

    chunk_preview.new = [
      range.point.x as i64,
      range.point.y as i64,
      range.point.z as i64,
    ];

    let mut chunk = Chunk::default();
    let chunk_pos = chunk.octree.get_size() / 2;

    for x in min..max {
      for y in min..max {
        for z in min..max {
          let pos = [
            range.point.x as i64 + x,
            range.point.y as i64 + y,
            range.point.z as i64 + z
          ];

          game_res.preview_chunk_manager.set_voxel2(&pos, voxel);
        }
      }
    }

    let min_prev = min - 2;
    let max_prev = max + 2;
    for x in min_prev..max_prev {
      for y in min_prev..max_prev {
        for z in min_prev..max_prev {
          let pos = [
            range.point.x as i64 + x,
            range.point.y as i64 + y,
            range.point.z as i64 + z
          ];

          let v = game_res.preview_chunk_manager.get_voxel(&pos);

          let local_x = chunk_pos as i64 + x;
          let local_y = chunk_pos as i64 + y;
          let local_z = chunk_pos as i64 + z;
          chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, v);
        }
      }
    }
    chunk_preview.chunk_op = Some(chunk);
  }
}


fn on_remove(
  mut commands: Commands,
  mut removed: RemovedComponents<Player>,
) {
  for entity in &mut removed {

  }
}



#[derive(Component, Clone)]
pub struct ChunkPreview {
  pub target: [i64; 3],
  pub new: [i64; 3],
  pub chunk_op: Option<Chunk>,
  pub is_showing: bool,
}

impl Default for ChunkPreview {
  fn default() -> Self {
    Self {
      target: [i64::MAX; 3],
      new: [i64::MAX; 3],
      chunk_op: None,
      is_showing: true,
    }
  }
}


#[derive(Resource)]
struct LocalResource {
  selected_keycode: KeyCode,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      selected_keycode: KeyCode::Key1,
    }
  }
}



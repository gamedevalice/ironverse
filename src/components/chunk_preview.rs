use bevy::prelude::*;
use voxels::chunk::{chunk_manager::Chunk};
use crate::{data::{GameResource}, utils::{nearest_voxel_point_0, nearest_voxel_point}, input::hotbar::{HotbarResource, self}};
use super::{raycast::Raycast, range::Range, player::Player};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(add)
      // .add_system(on_raycast)
      .add_system(on_range)
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

fn on_raycast(
  mut game_res: ResMut<GameResource>,
  mut raycasts: Query<
  (Entity, &Raycast, &mut ChunkPreview), Changed<Raycast>
  >,

  hotbar_res: Res<HotbarResource>,
) {


  for (_entity, raycast, mut chunk_preview) in &mut raycasts {
    info!("raycasts");
    if raycast.point.x == f32::NAN {
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();

    let nearest_op = nearest_voxel_point_0(
      &game_res.chunk_manager, 
      raycast.point, 
      true
    );

    if nearest_op.is_none() { continue; }
    let target = nearest_op.unwrap();
    if chunk_preview.target != target {
      chunk_preview.target = target;

      let new_op = nearest_voxel_point(
        &game_res.chunk_manager, 
        raycast.point, 
        true,
        0
      );
    }


    
    let new_op = nearest_voxel_point(
      &game_res.chunk_manager, 
      raycast.point, 
      true,
      0
    );

    if new_op.is_none() { continue; }
    let new = new_op.unwrap();

    if chunk_preview.new != new {
      info!("create");
      chunk_preview.new = new.clone();

      let bar_op = hotbar_res
        .bars
        .iter()
        .find(|bar| bar.key_code == hotbar_res.selected_keycode);

      let mut voxel = 0;
      if bar_op.is_some() {
        voxel = bar_op.unwrap().voxel;
      }


      let _ = game_res.preview_chunk_manager.set_voxel2(&new, voxel);

      let mut chunk = Chunk::default();
      let pos = chunk.octree.get_size() / 2;

      let range = 2;
      for x in -range..range + 1 {
        for y in -range..range + 1 {
          for z in -range..range + 1{
            let p_x = new[0] + x;
            let p_y = new[1] + y;
            let p_z = new[2] + z;

            let val = game_res.preview_chunk_manager.get_voxel(&[p_x, p_y, p_z]);

            let local_x = pos as i64 + x;
            let local_y = pos as i64 + y;
            let local_z = pos as i64 + z;
            chunk.octree.set_voxel(local_x as u32, local_y as u32, local_z as u32, val);
          }
        }
      }

      chunk_preview.chunk_op = Some(chunk);
    }
  }
}


fn on_range(
  mut game_res: ResMut<GameResource>,
  mut ranges: Query<
    (&Range, &mut ChunkPreview), Changed<Range>
  >,
  hotbar_res: Res<HotbarResource>,
) {
  for (range, mut chunk_preview) in &mut ranges {
    if range.point.x.is_nan() {
      chunk_preview.chunk_op = None;
      continue;
    }

    let size = 2_u32.pow(range.scale as u32);

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();
    
    let mut min = -(range.scale as i64);
    let mut max = (range.scale as i64);
    if size == 1 {
      max = 1;
    }
    let normal_mode = true;
    
    // if chunk_edit.snap_mode == SnapMode::Grid {
    if normal_mode {
      min = 0;
      max = size as i64;
    }

    // info!("size {} range.scale {} min max: {} {}", size, range.scale, min, max);

    chunk_preview.new = [
      range.point.x as i64,
      range.point.y as i64,
      range.point.z as i64,
    ];

    let bar_op = hotbar_res
      .bars
      .iter()
      .find(|bar| bar.key_code == hotbar_res.selected_keycode);

    let mut voxel = 0;
    if bar_op.is_some() {
      voxel = bar_op.unwrap().voxel;
    } 

    let mut chunk = Chunk::default();
    let chunk_pos = chunk.octree.get_size() / 2;
    // info!("min {} max {} size {}", min, max, size);

    for x in min..max {
      for y in min..max {
        for z in min..max {
          let pos = [
            range.point.x as i64 + x,
            range.point.y as i64 + y,
            range.point.z as i64 + z
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



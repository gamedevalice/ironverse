use bevy::prelude::*;
use voxels::chunk::chunk_manager::Chunk;
use crate::{data::GameResource, components::chunk_edit::{ChunkEditParams, ChunkEdit, get_point_by_edit_mode, EditState}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(
        update_position
          .run_if(add_normal)
      )
      .add_system(
        position_changed
          .after(update_position)
          .run_if(add_normal)
      );
  }
}

fn add_normal(state: Res<State<EditState>>) -> bool {
  state.0 == EditState::AddNormal
}

fn update_position(
  game_res: Res<GameResource>,
  mut chunk_edits: Query<(&Transform, &ChunkEditParams, &mut ChunkEdit)>,
) {
  for (trans, params, mut edit) in &mut chunk_edits {
    let mut pos_op = None;
    let total_div = (params.dist * 2.0) as i64;
    let min_dist = params.size as f32 * 2.0;

    'main: for i in (0..total_div).rev() {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (params.dist / div_f32) * i as f32;
      if dist < min_dist {
        continue;
      }

      // let p = get_point_by_edit_mode(&trans, dist, params.size, false);
      let t = trans.translation;
      let f = trans.forward();
      let p = utils::RayUtils::get_normal_point_with_scale(
        [t.x, t.y, t.z], [f.x, f.y, f.z], dist, game_res.voxel_scale
      );
      let mul = 1.0 / game_res.voxel_scale;
      let voxel_x = (p[0] * mul) as i64;
      let voxel_y = (p[1] * mul) as i64;
      let voxel_z = (p[2] * mul) as i64;
      
      let p_i64 = [voxel_x, voxel_y, voxel_z];
  
      let res = game_res.chunk_manager.get_voxel_safe(&p_i64);
      if res.is_some() && res.unwrap() == 0 {
        pos_op = Some(Vec3::new(p[0], p[1], p[2]));
        break 'main;
      }

    }

    if pos_op.is_none() {
      if edit.position.is_some() {
        edit.position = None;
      }
      continue;
    }
    let pos = pos_op.unwrap();

    if edit.position.is_some() {
      let p = edit.position.unwrap();

      if p != pos {
        edit.position = Some(pos);
      }
    }

    if edit.position.is_none() {
      edit.position = Some(pos);
    }

    
  }
}

fn position_changed(
  mut edits: Query<
  (&mut ChunkEdit, &ChunkEditParams), 
  Or<(Changed<ChunkEdit>, Changed<ChunkEditParams>)>
  >,
  mut game_res: ResMut<GameResource>,
) {
  for (mut edit, params) in &mut edits {
    if edit.position.is_none() {
      edit.chunk = None;
      continue;
    }

    game_res.preview_chunk_manager.chunks = game_res.chunk_manager.chunks.clone();
    
    let min = 0;
    let max = params.size as i64;

    let point = edit.position.unwrap();
    for x in min..max {
      for y in min..max {
        for z in min..max {
          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];

          let _ = game_res.preview_chunk_manager.set_voxel2(&pos, params.voxel);
        }
      }
    }

    let min_prev = min - 2;
    let max_prev = max + 2;
    let mut chunk = Chunk::default();
    let chunk_pos = (chunk.octree.get_size() / 2) as f32 * game_res.voxel_scale;
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
    // chunk.octree.set_voxel(2, 2, 2, 1);

    edit.chunk = Some(chunk);
  }
}


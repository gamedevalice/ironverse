use bevy::prelude::*;
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Isometry, ColliderHandle};
use rapier3d::geometry::Group;
use voxels::chunk::chunk_manager::Chunk;
use voxels::data::voxel_octree::MeshData;
use voxels::{chunk::{chunk_manager::ChunkManager, adjacent_keys}, data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::data::GameResource;
use crate::physics::Physics;
use super::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(on_player_add)
      .add_system(on_player_move);
  }
}

fn on_player_add(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,

  mut player_query: Query<(Entity, &Player), Added<Player>>,
) {
  
  for (entity, player) in &mut player_query {
    let config = game_res.chunk_manager.config.clone();
    let keys = adjacent_keys(&player.key, 1, true);

    let mut chunks = Chunks { data: Vec::new() };

    for key in keys.iter() {
      let test_key = [[0, -1, -2], [0, -1, -1], [0, -1, 0]];
      if !test_key.contains(key) {
        // continue;
      }


      let mut chunk = Chunk::default();
      let chunk_op = game_res.chunk_manager.get_chunk(key);
      if chunk_op.is_some() {
        chunk = chunk_op.unwrap().clone();
      } else {
        chunk = ChunkManager::new_chunk(
          key, 
          config.depth, 
          config.lod, 
          game_res.chunk_manager.noise,
        );
      }
  
      let data = chunk.octree.compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse.clone(),
        &game_res.colors,
        1.0,
      );

      game_res.chunk_manager.set_chunk(key, &chunk);

      if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
        continue;
      }
      
      let pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
      let mut pos = Vec::new();
      for d in data.positions.iter() {
        pos.push(Point::from([d[0], d[1], d[2]]));
      }
  
      let mut indices = Vec::new();
      for ind in data.indices.chunks(3) {
        // println!("i {:?}", ind);
        indices.push([ind[0], ind[1], ind[2]]);
      }
  
      let mut collider = ColliderBuilder::trimesh(pos, indices)
        .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
        .build();
      collider.set_position(Isometry::from(pos_f32));
  
      let handle = physics.collider_set.insert(collider);

      chunks.data.push(Mesh {
        key: key.clone(),
        data: data.clone(),
        chunk: chunk.clone(),
        handle: handle,
      });

      
    }

    commands
      .entity(entity)
      .insert(chunks);

    info!("chunks");
  }
}

/*
  0.0, 2.0, -13.0
  0.0, 2.0, -14.0
 */

fn on_player_move(
  mut commands: Commands,
  mut players: Query<(Entity, &Player, &mut Chunks), Changed<Player>>,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,
) {
  for (_entity, player, mut chunks) in &mut players {
    if player.key == player.prev_key {
      continue;
    }

    let config = game_res.chunk_manager.config.clone();
    let keys = adjacent_keys(&player.key, 1, true);
    for i in 0..chunks.data.len() {
      let m = &chunks.data[i];

      physics.remove_collider(m.handle);
    }
    chunks.data.clear();

    for key in keys.iter() {
      let mut chunk = Chunk::default();
      let chunk_op = game_res.chunk_manager.get_chunk(key);
      if chunk_op.is_some() {
        chunk = chunk_op.unwrap().clone();
      } else {
        chunk = ChunkManager::new_chunk(
          key, 
          config.depth, 
          config.lod, 
          game_res.chunk_manager.noise,
        );
      }

      let data = chunk.octree.compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse.clone(),
        &game_res.colors,
        1.0,
      );

      game_res.chunk_manager.set_chunk(key, &chunk);

      if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
        continue;
      }
      
      let pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
      let mut pos = Vec::new();
      for d in data.positions.iter() {
        pos.push(Point::from([d[0], d[1], d[2]]));
      }
  
      let mut indices = Vec::new();
      for ind in data.indices.chunks(3) {
        // println!("i {:?}", ind);
        indices.push([ind[0], ind[1], ind[2]]);
      }
  
      let mut collider = ColliderBuilder::trimesh(pos, indices)
        .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
        .build();
      collider.set_position(Isometry::from(pos_f32));
  
      let handle = physics.collider_set.insert(collider);

      chunks.data.push(Mesh {
        key: key.clone(),
        data: data.clone(),
        chunk: chunk.clone(),
        handle: handle,
      });
    }



  }


}



#[derive(Component)]
pub struct Chunks {
  pub data: Vec<Mesh>,
}


#[derive(Component, Debug, Clone)]
pub struct Mesh {
  pub key: [i64; 3],
  pub chunk: Chunk,
  pub data: MeshData,
  pub handle: ColliderHandle,
}

impl Default for Chunks {
  fn default() -> Self {
    Self {
      data: Vec::new()
    }
  }
}




#[cfg(test)]
mod tests {
  use voxels::chunk::voxel_pos_to_key;

  #[test]
  fn test() -> Result<(), String> {
    let key = voxel_pos_to_key(&[0, 2, -13], 12);
    println!("key {:?}", key);
    Ok(())
  }
}
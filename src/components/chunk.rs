use bevy::prelude::*;
use bevy_voxel::BevyVoxelResource;
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Isometry, ColliderHandle};
use rapier3d::geometry::Group;
use voxels::chunk::adj_keys_by_scale;
use voxels::chunk::chunk_manager::Chunk;
use voxels::data::voxel_octree::MeshData;
use voxels::{chunk::{chunk_manager::ChunkManager, adjacent_keys}, data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::data::GameResource;
use super::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_system(on_player_add)
      // .add_system(on_player_move)
      ;
  }
}

fn on_player_add(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,

  mut player_query: Query<(Entity, &Player), Added<Player>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for (entity, player) in &mut player_query {
    println!("player add");
    let all_chunks = bevy_voxel_res.load_adj_chunks_with_collider(player.key);
    let mut chunks = Chunks { data: Vec::new() };

    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }

      let pos = bevy_voxel_res.get_pos(chunk.key);
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }

    commands
      .entity(entity)
      .insert(chunks); 
  }
}

fn on_player_move(
  mut commands: Commands,
  mut players: Query<(Entity, &Player, &mut Chunks), Changed<Player>>,
  mut game_res: ResMut<GameResource>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for (_entity, player, mut chunks) in &mut players {
    if player.key == player.prev_key {
      continue;
    }

    let all_chunks = bevy_voxel_res.load_adj_chunks(player.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }

      let pos = bevy_voxel_res.get_pos(chunk.key);
      let handle = bevy_voxel_res.add_collider(pos, &data);
      
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }


  // for (_entity, player, mut chunks) in &mut players {
  //   if player.key == player.prev_key {
  //     continue;
  //   }
    
  //   let scale = game_res.voxel_scale;
  //   let mul = (1.0 / scale) as i64;
  //   let config = game_res.chunk_manager.config.clone();
  //   for i in 0..chunks.data.len() {
  //     let m = &chunks.data[i];

  //     physics.remove_collider(m.handle);
  //   }
  //   chunks.data.clear();

  //   let keys = adj_keys_by_scale(player.key, 1, scale);
  //   for key in keys.iter() {
  //     let mut chunk = Chunk::default();
  //     let chunk_op = game_res.chunk_manager.get_chunk(key);
  //     if chunk_op.is_some() {
  //       chunk = chunk_op.unwrap().clone();
  //     } else {
  //       chunk = ChunkManager::new_chunk(
  //         key, 
  //         config.depth, 
  //         config.lod, 
  //         game_res.chunk_manager.noise,
  //       );
  //     }

  //     let data = chunk.octree.compute_mesh(
  //       VoxelMode::SurfaceNets, 
  //       &mut game_res.chunk_manager.voxel_reuse.clone(),
  //       &game_res.colors,
  //       game_res.voxel_scale,
  //     );

  //     game_res.chunk_manager.set_chunk(key, &chunk);

  //     if data.indices.len() == 0 { // Temporary, should be removed once the ChunkMode detection is working
  //       continue;
  //     }
      
  //     let mut pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
  //     pos_f32[0] *= scale;
  //     pos_f32[1] *= scale;
  //     pos_f32[2] *= scale;

  //     let mut pos = Vec::new();
  //     for d in data.positions.iter() {
  //       pos.push(Point::from([d[0], d[1], d[2]]));
  //     }
  
  //     let mut indices = Vec::new();
  //     for ind in data.indices.chunks(3) {
  //       // println!("i {:?}", ind);
  //       indices.push([ind[0], ind[1], ind[2]]);
  //     }
  
  //     let mut collider = ColliderBuilder::trimesh(pos, indices)
  //       .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
  //       .build();
  //     collider.set_position(Isometry::from(pos_f32));
  
  //     let handle = physics.collider_set.insert(collider);

  //     chunks.data.push(Mesh {
  //       key: key.clone(),
  //       data: data.clone(),
  //       chunk: chunk.clone(),
  //       handle: handle,
  //     });
  //   }



  // }


}



#[derive(Component)]
pub struct Chunks {
  pub data: Vec<ChunkData>,
}


#[derive(Component, Debug, Clone)]
pub struct ChunkData {
  pub key: [i64; 3],
  pub data: MeshData,
}

impl Default for Chunks {
  fn default() -> Self {
    Self {
      data: Vec::new(),
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
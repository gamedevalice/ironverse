use bevy::prelude::*;
use bevy_flycam::FlyCam;
use voxels::{data::voxel_octree::VoxelOctree, chunk::chunk_manager::Chunk};
use crate::{data::{GameResource, GameState, UIState}, physics::Physics, graphics::ChunkGraphics, components::player::Player};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Load), enter);
  }
}

fn enter(
  mut commands: Commands,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,
  player_query: Query<(Entity, &Player)>,
  mut game_state_next: ResMut<NextState<GameState>>,
  mut ui_state_next: ResMut<NextState<UIState>>,


  terrain_graphics: Query<Entity, With<ChunkGraphics>>,

  cameras: Query<Entity, With<FlyCam>>,
) {
  game_res.chunk_manager.chunks.clear();
  *physics = Physics::default();
  
  for (entity, _) in &player_query {
    commands.entity(entity).despawn_recursive();
  }
  
  for entity in &terrain_graphics {
    commands.entity(entity).despawn_recursive();
  }

  for entity in &cameras {
    commands.entity(entity).despawn_recursive();
  }


  let data = game_res.data.clone();
  for i in 0..data.terrains.keys.len() {
    let key = &data.terrains.keys[i];
    let voxels_str = &data.terrains.voxels[i];
    let voxels_res = array_bytes::hex2bytes(voxels_str);
    if voxels_res.is_ok() {
      let data = voxels_res.unwrap();
      let octree = VoxelOctree::new_from_bytes(data);
      let chunk = Chunk {
        key: key.clone(),
        octree: octree,
        is_default: false,
        ..Default::default()
      };
      game_res.chunk_manager.set_chunk(key, &chunk);

      // info!("load data key {:?}", key);
    }
  }



  ui_state_next.set(UIState::Default);
  game_state_next.set(GameState::Init);
  info!("Enter GameState::Load");
}

use bevy::prelude::{*, Component, Vec3};
use voxels::chunk::chunk_manager::Chunk;
use self::player::Player;

pub mod camera;
pub mod chunk;
// pub mod chunk_edit;
pub mod player;
// pub mod raycast;
// pub mod range;
// pub mod chunk_preview;

pub mod voxel_add;


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(voxel_add::CustomPlugin);

    app
      .add_system(add_to_player);
  }
}

fn add_to_player(
  mut commands: Commands,
  player_query: Query<Entity, Added<Player>>,
) {
  for entity in &player_query {
    commands
      .entity(entity)
      .insert(ChunkEdit::default())
      .insert(ChunkEditParams::default());
  }
}


/* Helper functions */
fn get_snapped_position(pos: Vec3, size: u32) -> Vec3 {
  let adj_positions = get_nearby_snapped_positions(pos, size);

  let mut min_dist = f32::MAX;
  let mut snapped_pos = Vec3::ZERO;
  for adj_pos in adj_positions.iter() {
    let dist = pos.distance_squared(*adj_pos);

    if dist < min_dist {
      min_dist = dist;
      snapped_pos = *adj_pos;
    }
  }

  snapped_pos
}

fn get_nearby_snapped_positions(pos: Vec3, size: u32) -> Vec<Vec3> {
  let mut result = Vec::new();

  let size_i64 = size as i64;
  let base_x = ( (pos.x as i64) / size_i64 ) * size_i64;
  let base_y = ( (pos.y as i64) / size_i64 ) * size_i64;
  let base_z = ( (pos.z as i64) / size_i64 ) * size_i64;

  // println!("base_x {}", base_x);

  let range = 1;
  let min = -range;
  let max = range + 1;
  for x in min..max {
    for y in min..max {
      for z in min..max {
        let adj_x = base_x + (x * size_i64);
        let adj_y = base_y + (y * size_i64);
        let adj_z = base_z + (z * size_i64);

        result.push(Vec3::new(adj_x as f32, adj_y as f32, adj_z as f32));

        // println!("adj_x {}", adj_x);
      }
    }
  }
  

  result
}

fn get_point_by_edit_mode(
  trans: &Transform, dist: f32, size: u32, snap_to_grid: bool
) -> Vec3 {
  let mut point = trans.translation + trans.forward() * dist;
  point -= (size as f32 * 0.5 - 0.5);

  let mut s = size;
  if !snap_to_grid {
    s = 1;
  }
  get_snapped_position(point, s)
}






#[derive(Default, Component)]
pub struct ChunkEdit {
  pub position: Option<Vec3>,
  pub chunk: Option<Chunk>,
}

#[derive(Component)]
pub struct ChunkEditParams {
  pub level: u8,
  pub dist: f32,

  pub size: u32, 
}

impl Default for ChunkEditParams {
  fn default() -> Self {
    let level = 1;
    Self {
      level: level,
      dist: 8.0,
      size: 2_u32.pow(level as u32),
    }
  }
}


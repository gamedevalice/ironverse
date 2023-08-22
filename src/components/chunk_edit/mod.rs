use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_voxel::{Selected, Preview, SelectedGraphics, BevyVoxelResource, PreviewGraphics};
use voxels::{chunk::chunk_manager::Chunk, data::voxel_octree::VoxelMode};
use crate::input::hotbar::HotbarResource;

use super::{player::Player, chunk::{Chunks}};

// mod voxel_add;
// mod voxel_remove;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<EditState>()
      // .add_plugin(voxel_add::CustomPlugin)
      // .add_plugin(voxel_remove::CustomPlugin)
      .add_system(add_to_player)
      // .add_system(update_edit_params)
      .add_system(switch_state)
      .add_system(add_voxel);
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
      .insert(ChunkEditParams::default())
      .insert(Selected::default())
      .insert(Preview::default());

    commands
      .spawn(SelectedGraphics)
      .insert(PreviewGraphics);
  }
}

fn update_edit_params(
  mut mouse_wheels: EventReader<MouseWheel>,
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut chunk_edit_params: Query<&mut ChunkEditParams>,

  hotbar_res: Res<HotbarResource>,
) {
  for event in mouse_wheels.iter() {
    for mut params in chunk_edit_params.iter_mut() {
      // Need to clamp as event.y is returning -120.0 to 120.0 (Bevy bug)
      let seamless_size = 12 as f32;
      let adj = 12.0;
      let limit = seamless_size + adj;
      if params.dist <= limit {
        params.dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
      }
      
      if params.dist > limit {
        params.dist = limit;
      }

      let size = 2_u32.pow(params.level as u32);
      let min_val = size as f32;
      if params.dist < min_val {
        params.dist = min_val;
      }
    }
  }

  if key_input.just_pressed(KeyCode::Equals) {
    for mut params in chunk_edit_params.iter_mut() {
      if params.level < 3 {
        params.level += 1;
        params.size = 2_u32.pow(params.level as u32);
      }
    }
  }

  if key_input.just_pressed(KeyCode::Minus) {
    for mut params in chunk_edit_params.iter_mut() {
      if params.level > 0 {
        params.level -= 1;
        params.size = 2_u32.pow(params.level as u32);
      }
    }
  }

  for i in 0..hotbar_res.bars.len() {
    let bar = &hotbar_res.bars[i];
    let voxel = bar.voxel;

    if bar.key_code == hotbar_res.selected_keycode {
      for mut params in chunk_edit_params.iter_mut() {
        if params.voxel != voxel {
          params.voxel = voxel;
        }
      }
    }
  }
    
}

fn switch_state(
  key_input: Res<Input<KeyCode>>,
  mut next_state: ResMut<NextState<EditState>>,
  state: Res<State<EditState>>,
) {

  if key_input.just_pressed(KeyCode::M) {
    match state.0 {
      EditState::AddNormal => {
        next_state.set(EditState::AddSnap);
      },
      EditState::AddSnap | _ => {
        next_state.set(EditState::AddNormal);
      }
    }
  }

  if key_input.just_pressed(KeyCode::N) {
    match state.0 {
      EditState::RemoveNormal => {
        next_state.set(EditState::RemoveSnap);
      },
      EditState::RemoveSnap | _ => {
        next_state.set(EditState::RemoveNormal);
      }
    }
  }
}


fn add_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Player, &mut Chunks)>,
) {
  let mut voxel = None;
  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(1);
  }
  if voxel.is_none() {
    return;
  }

  for (preview, player, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
    let p = preview.pos.unwrap();
    let pos = bevy_voxel_res.get_nearest_voxel_air(p).unwrap();

    bevy_voxel_res.set_voxel(pos, voxel.unwrap());

    let all_chunks = bevy_voxel_res.load_adj_chunks(player.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }

      let pos = bevy_voxel_res.get_pos(chunk.key);
      let handle = bevy_voxel_res.add_collider(pos, &data);
      
      chunks.data.push(super::chunk::Mesh {
        key: chunk.key.clone(),
        data: data.clone(),
        chunk: chunk.clone(),
        handle: handle,
      });
    }
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
  pub voxel: u8,

  pub size: u32,
}

impl Default for ChunkEditParams {
  fn default() -> Self {
    let level = 1;
    Self {
      level: level,
      dist: 8.0,
      voxel: 0,
      size: 2_u32.pow(level as u32),
    }
  }
}


#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum EditState {
  #[default]
  AddNormal,
  AddSnap,
  RemoveNormal,
  RemoveSnap,
}
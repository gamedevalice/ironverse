use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy_voxel::{Selected, Preview, SelectedGraphics, BevyVoxelResource, PreviewGraphics, Center, Chunks, EditState};
use rapier3d::prelude::ColliderHandle;
use voxels::{chunk::chunk_manager::Chunk, data::voxel_octree::VoxelMode};
use crate::{input::hotbar::HotbarResource, graphics::ChunkGraphics};

use super::player::Player;

// mod voxel_add;
// mod voxel_remove;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_plugin(voxel_add::CustomPlugin)
      // .add_plugin(voxel_remove::CustomPlugin)
      .add_system(add_to_player)
      // .add_system(update_edit_params)
      .add_system(switch_state);
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
      .insert(Chunks::default())
      .insert(Preview::default());

    commands
      .spawn(SelectedGraphics)
      .insert(PreviewGraphics);
  }
}

fn switch_state(
  key_input: Res<Input<KeyCode>>,
  mut next_state: ResMut<NextState<EditState>>,
  state: Res<State<EditState>>,
) {

  if key_input.just_pressed(KeyCode::M) {
    next_state.set(EditState::AddNormal);
    println!("EditState::AddNormal");
    // match state.0 {
    //   EditState::AddNormal => {
    //     next_state.set(EditState::AddSnap);
    //   },
    //   EditState::AddSnap | _ => {
    //     next_state.set(EditState::AddNormal);
    //   }
    // }
  }

  if key_input.just_pressed(KeyCode::N) {
    next_state.set(EditState::RemoveNormal);
    println!("EditState::RemoveNormal");
    // match state.0 {
    //   EditState::RemoveNormal => {
    //     next_state.set(EditState::RemoveSnap);
    //   },
    //   EditState::RemoveSnap | _ => {
    //     next_state.set(EditState::RemoveNormal);
    //   }
    // }
  }
}



/* fn update_edit_params(
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
 */


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
 

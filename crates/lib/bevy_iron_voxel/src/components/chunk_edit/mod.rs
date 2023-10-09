use bevy::prelude::*;
use bevy_voxel::{Selected, Preview, SelectedGraphics, PreviewGraphics, Chunks, EditState, ShapeState, MeshComponent};
use voxels::chunk::chunk_manager::Chunk;
use super::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_plugin(voxel_add::CustomPlugin)
      // .add_plugin(voxel_remove::CustomPlugin)
      .add_systems(Update, add_to_player)
      // .add_system(update_edit_params)
      .add_systems(Update, switch_state);
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
      .insert(MeshComponent::default())
      .insert(Preview::default())
      ;

    commands
      .spawn(SelectedGraphics)
      .insert(PreviewGraphics);
  }
}

fn switch_state(
  _key_input: Res<Input<KeyCode>>,
  mut _next_edit: ResMut<NextState<EditState>>,
  _state: Res<State<EditState>>,

  mut _next_shape: ResMut<NextState<ShapeState>>,
  _shape_state: Res<State<ShapeState>>,

  mut _local: Local<usize>,
  mut _local1: Local<usize>,
) {

  // if key_input.just_pressed(KeyCode::Down) {
  //   let len = EditState::variants().len();
  //   *local1 += 1;
  //   *local1 = *local1 % len;

  //   for (i, state) in EditState::variants().enumerate() {
  //     if *local1 == i {
  //       next_edit.set(state);

  //       println!("EditState {:?}", state);
  //       break;
  //     }
  //   }
  // }

  // if key_input.just_pressed(KeyCode::Up) {
  //   let len = ShapeState::variants().len();
  //   *local += 1;
  //   *local = *local % len;

  //   for (i, state) in ShapeState::variants().enumerate() {
  //     if *local == i {
  //       next_shape.set(state);

  //       println!("ShapeState {:?}", state);
  //       break;
  //     }
  //   }
  // }
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
 

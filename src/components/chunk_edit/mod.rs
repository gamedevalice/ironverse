use bevy::prelude::*;

use self::create_normal::CreateNormal;

use super::player::Player;

mod create_normal;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ChunkEditResource::default())
      .add_plugin(create_normal::CustomPlugin)
      .add_system(add)
      .add_system(manage_modes);
  }
}

fn add(
  mut commands: Commands,
  player_added: Query<Entity, Added<Player>>,
) {
  for entity in &player_added {
    commands
      .entity(entity)
      .insert(CreateNormal::default());
  }
}


fn manage_modes(
  mut commands: Commands,
  mut chunk_edit_res: ResMut<ChunkEditResource>,
  key_input: Res<Input<KeyCode>>,
  players: Query<Entity, With<Player>>,
) {

  if key_input.just_pressed(KeyCode::M) {
    for entity in &players {
      commands.entity(entity).remove::<CreateNormal>();
      // Add the remaining 3 mode here
    }


    match chunk_edit_res.edit_mode {
      EditMode::CreateNormal => {
        chunk_edit_res.edit_mode = EditMode::CreateSnap;
      },
      EditMode::CreateSnap => {
        chunk_edit_res.edit_mode = EditMode::DeleteNormal;
      },
      EditMode::DeleteNormal => {
        chunk_edit_res.edit_mode = EditMode::DeleteSnap;
      },
      EditMode::DeleteSnap => {
        chunk_edit_res.edit_mode = EditMode::CreateNormal;
      },
      // _ => {},
    };

    info!("Edit_mode {:?}", chunk_edit_res.edit_mode);
  }

}


#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum EditMode {
  CreateNormal,
  CreateSnap,
  DeleteNormal,
  DeleteSnap,
}

#[derive(Resource)]
pub struct ChunkEditResource {
  pub edit_mode: EditMode
}

impl Default for ChunkEditResource {
  fn default() -> Self {
    Self {
      edit_mode: EditMode::CreateNormal,
    }
  }
}





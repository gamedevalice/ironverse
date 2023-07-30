use bevy::{prelude::*, input::mouse::MouseWheel};
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
      .add_system(manage_modes)
      .add_system(update_edit_values);
  }
}

fn add(
  mut commands: Commands,
  player_added: Query<Entity, Added<Player>>,
) {
  for entity in &player_added {
    commands
      .entity(entity)
      .insert(ChunkEdit::default())
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


    for entity in &players {
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
          commands.entity(entity).insert(CreateNormal::default());
        },
        // _ => {},
      };
    }

    

    info!("Edit_mode {:?}", chunk_edit_res.edit_mode);
  }

}


fn update_edit_values(
  mut chunk_edits: Query<&mut ChunkEdit>,
  mut mouse_wheels: EventReader<MouseWheel>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  for event in mouse_wheels.iter() {
    for mut chunk_edit in chunk_edits.iter_mut() {
      // Need to clamp as event.y is returning -120.0 to 120.0 (Bevy bug)
      let seamless_size = 12 as f32;
      let adj = 12.0;
      let limit = seamless_size + adj;
      if chunk_edit.dist <= limit {
        chunk_edit.dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
      }
      
      if chunk_edit.dist > limit {
        chunk_edit.dist = limit;
      }


      let size = 2_u32.pow(chunk_edit.scale as u32);
      let min_val = size as f32;
      if chunk_edit.dist < min_val {
        chunk_edit.dist = min_val;
      }
      
    }
  }

  if keyboard_input.just_pressed(KeyCode::Equals) {
    for mut chunk_edit in chunk_edits.iter_mut() {
      if chunk_edit.scale < 3 {
        chunk_edit.scale += 1;
      }
    }
  }

  if keyboard_input.just_pressed(KeyCode::Minus) {
    for mut chunk_edit in chunk_edits.iter_mut() {
      if chunk_edit.scale > 0 {
        chunk_edit.scale -= 1;
        // info!("range.scale {}", range.scale);
      }
      
    }
  }
}


#[derive(Component)]
pub struct ChunkEdit {
  pub point_op: Option<Vec3>,
  pub dist: f32,
  pub scale: u8,
  pub min: i64,
  pub max: i64,
  pub voxel : u8,
}

impl Default for ChunkEdit {
  fn default() -> Self {
    Self {
      point_op: None,
      dist: 8.0,
      scale: 1,
      min: 0,
      max: 0,
      voxel: 0,
    }
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





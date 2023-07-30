use bevy::{prelude::*, input::{mouse::MouseWheel, ButtonState}, utils::HashMap};
use rapier3d::{na::{Point, Isometry}, prelude::{ColliderBuilder, InteractionGroups, Group}};
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::{input::{MouseInput, hotbar::HotbarResource}, data::GameResource, physics::Physics};

use self::create_normal::CreateNormal;
use super::{player::Player, chunk::{Chunks, Mesh}};
mod create_normal;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ChunkEditResource::default())
      .add_plugin(create_normal::CustomPlugin)
      .add_system(add)
      .add_system(manage_modes)
      .add_system(update_edit_values)
      .add_system(edit);
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

fn edit(
  mut mouse_inputs: EventReader<MouseInput>,
  hotbar_res: Res<HotbarResource>,
  mut game_res: ResMut<GameResource>,
  mut physics: ResMut<Physics>,

  mut edits: Query<(&ChunkEdit, &mut Chunks)>,
) {
  let mut voxel_op = None;
  for event in mouse_inputs.iter() {
    if event.mouse_button_input.state == ButtonState::Pressed 
    && event.mouse_button_input.button == MouseButton::Left {
      voxel_op = Some(1);
      for i in 0..hotbar_res.bars.len() {
        let bar = &hotbar_res.bars[i];
        if  hotbar_res.selected_keycode ==  bar.key_code {
          voxel_op = Some(bar.voxel);
        }

      }
    }
  }
  if voxel_op.is_none() {
    return;
  }

  for (edit, mut chunks) in &mut edits {
    if edit.point_op.is_none() { continue; }

    let point = edit.point_op.unwrap();

    let mut res = HashMap::new();
    for x in edit.min..edit.max {
      for y in edit.min..edit.max {
        for z in edit.min..edit.max {

          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];
          let chunks = game_res.chunk_manager.set_voxel2(&pos, voxel_op.unwrap());
          for (key, chunk) in chunks.iter() {
            res.insert(key.clone(), chunk.clone());
          }
        }
      }
    }

    let config = game_res.chunk_manager.config.clone();
    for (key, chunk) in res.iter() {
      'inner: for i in 0..chunks.data.len() {
        let m = &chunks.data[i];

        if key == &m.key {
          physics.remove_collider(m.handle);
          chunks.data.swap_remove(i);
          break 'inner;
        }
      }
      

      let data = chunk.octree.compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut game_res.chunk_manager.voxel_reuse
      );

      
      if data.indices.len() > 0 {
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


        let mut c = chunk.clone();
        c.is_default = false;
        chunks.data.push(Mesh {
          key: key.clone(),
          chunk: c,
          data: data.clone(),
          handle: handle,
        });
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





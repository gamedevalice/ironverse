use bevy::{prelude::*, input::{mouse::MouseWheel, ButtonState}, utils::HashMap};
use rapier3d::prelude::{Point, ColliderBuilder, InteractionGroups, Group, Isometry};
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32};
use crate::{data::GameResource, input::{MouseInput, hotbar::HotbarResource}, physics::Physics, components::EditMode};
use self::{normal::Normal, snap::SnapGrid};
use super::{ChunkEdit, ChunkEditParams, chunk::{Chunks, Mesh}, player::Player, ChunkEditResource};

mod normal;
mod snap;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(normal::CustomPlugin)
      .add_plugin(snap::CustomPlugin)
      .add_system(add)
      .add_system(switch_modes)
      .add_system(update_edit_params)
      .add_system(on_edit);
  }
}

fn add(
  mut commands: Commands,
  player_query: Query<Entity, Added<Player>>,
) {
  for entity in &player_query {
    commands
      .entity(entity)
      // .insert(Normal::default())
      .insert(SnapGrid::default())
      ;
  }
}

fn switch_modes(
  mut commands: Commands,
  key_input: Res<Input<KeyCode>>,
  players: Query<Entity, With<Player>>,

  mut chunk_edit_res: ResMut<ChunkEditResource>,
) {

  if key_input.just_pressed(KeyCode::M) {
    match chunk_edit_res.edit_mode {
      EditMode::CreateNormal => {
        chunk_edit_res.edit_mode = EditMode::CreateSnap;
        for entity in &players {
          commands.entity(entity).remove::<Normal>();
          commands.entity(entity).insert(SnapGrid::default());
        }
      },
      EditMode::CreateSnap => {
        chunk_edit_res.edit_mode = EditMode::CreateNormal;
        for entity in &players {
          commands.entity(entity).remove::<SnapGrid>();
          commands.entity(entity).insert(Normal::default());
        }
      },
      _ => {
        chunk_edit_res.edit_mode = EditMode::CreateNormal;
        for entity in &players {
          commands.entity(entity).remove::<SnapGrid>();
          commands.entity(entity).insert(Normal::default());
        }
      },
    };

    info!("Edit_mode {:?}", chunk_edit_res.edit_mode);
  }
}





fn update_edit_params(
  mut mouse_wheels: EventReader<MouseWheel>,
  key_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut chunk_edit_params: Query<&mut ChunkEditParams>,
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
}

fn on_edit(
  hotbar_res: Res<HotbarResource>,

  mut game_res: ResMut<GameResource>,
  mut mouse_inputs: EventReader<MouseInput>,
  mut physics: ResMut<Physics>,
  mut edits: Query<(&ChunkEdit, &ChunkEditParams, &mut Chunks)>,
) {
  let mut edit_chunk = false;
  for event in mouse_inputs.iter() {
    if event.mouse_button_input.state == ButtonState::Pressed 
    && event.mouse_button_input.button == MouseButton::Left {
      edit_chunk = true;
    }
  }
  if !edit_chunk {
    return;
  }

  let mut voxel = 0;
  for i in 0..hotbar_res.bars.len() {
    let bar = &hotbar_res.bars[i];
    if  hotbar_res.selected_keycode == bar.key_code {
      voxel = bar.voxel;
    }
  }

  for (edit, params, mut chunks) in &mut edits {
    if edit.position.is_none() { continue; }

    let point = edit.position.unwrap();
    let mut res = HashMap::new();
    let min = 0;
    let max = params.size as i64;
    for x in min..max {
      for y in min..max {
        for z in min..max {

          let pos = [
            point.x as i64 + x,
            point.y as i64 + y,
            point.z as i64 + z
          ];
          let chunks = game_res.chunk_manager.set_voxel2(&pos, voxel);
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
        &mut game_res.chunk_manager.voxel_reuse.clone(),
        &game_res.colors,
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


struct LocalResource {
  
}


/*
  Define the components
    Preview
      Changeable based on selected voxel
      Add is different from remove
    Edit operation(Add)
      Different from remove
    Positioning
      Same with add and remove

  Treat everything as component
    To make it easier to modify
    Always treat the code as will be always be modified

  Main ingredients
    Selected voxel
    Defining the position
      Normal
      Snap to grid
    Size of chunk to edit
      Size of preview

  Data in/Data out
  Prefer top down approach than down to up when starting out the concept
    Prefer more control over encapsulation when implementing things
    Maximize control and transparency for now
      Be more conservative once it is established

  Centralized the data
  Show all the data
  Then treat behavior as a cartridge
  Then modularized it later on
*/















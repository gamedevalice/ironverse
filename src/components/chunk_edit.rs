use bevy::{prelude::*, input::ButtonState, utils::HashMap};
use rapier3d::prelude::{ColliderBuilder, InteractionGroups, Isometry, Point};
use voxels::{data::voxel_octree::VoxelMode, utils::key_to_world_coord_f32, chunk::chunk_manager::ChunkManager};
use crate::{physics::Physics, data::{GameResource}, utils::{nearest_voxel_point, nearest_voxel_point_0}, input::{MouseInput, hotbar::HotbarResource}};
use super::{raycast::Raycast, chunk::{Chunks, Mesh}, range::Range, player::Player};
use rapier3d::geometry::Group;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_system(update_by_terrain_hit)
      .add_system(add_to_player)
      .add_system(update_by_range)
      .add_system(toggle_snap_mode);
  }
}


fn add_to_player(
  mut commands: Commands,
  player_query: Query<Entity, Added<Player>>,
) {
  for entity in &player_query {
    commands
      .entity(entity)
      .insert(ChunkEdit::default());
  }
}

fn toggle_snap_mode(
  keys: Res<Input<KeyCode>>,
  mut chunk_edits: Query<&mut ChunkEdit>,
) {
  if keys.just_pressed(KeyCode::M) {
    for mut edit in &mut chunk_edits.iter_mut() {

      if edit.snap_mode == SnapMode::Normal {
        edit.snap_mode = SnapMode::Grid;
      } else {
        edit.snap_mode = SnapMode::Normal;
      }
      info!("SnapMode {:?}", edit.snap_mode);
    }
  }
}




fn update_by_terrain_hit(
  mut raycasts: Query<(Entity, &Raycast, &mut Chunks)>,
  mut game_res: ResMut<GameResource>,

  mut physics: ResMut<Physics>,
  hotbar_res: Res<HotbarResource>,
  mut mouse_inputs: EventReader<MouseInput>,
  
) {
  let mut voxel_op = None;
  for event in mouse_inputs.iter() {
    if event.mouse_button_input.state == ButtonState::Pressed {
      if event.mouse_button_input.button == MouseButton::Left {
        voxel_op = Some(0);
      }
      
      if event.mouse_button_input.button == MouseButton::Right {
        voxel_op = Some(1);
        for i in 0..hotbar_res.bars.len() {
          let bar = &hotbar_res.bars[i];
          if  hotbar_res.selected_keycode ==  bar.key_code {
            voxel_op = Some(bar.voxel);
          }
        }
      }
    }
  }

  if voxel_op.is_none() {
    return;
  }

  let config = game_res.chunk_manager.config.clone();
  for (_entity, raycast, mut chunks) in &mut raycasts {
    if raycast.point.x == f32::NAN {
      continue;
    }

    let mut res = Vec::new();
    let voxel = voxel_op.unwrap();

    // Delete
    if voxel == 0 {
      let nearest_op = nearest_voxel_point_0(
        &game_res.chunk_manager, 
        raycast.point, 
        true
      );
      if nearest_op.is_none() {
        continue;
      }
      res = game_res.chunk_manager.set_voxel2(&nearest_op.unwrap(), voxel);
    }

    // Add
    if voxel > 0 {
      let nearest_op = nearest_voxel_point(
        &game_res.chunk_manager, 
        raycast.point, 
        true,
        0
      );
  
      if nearest_op.is_none() {
        continue;
      }

      
      res = game_res.chunk_manager.set_voxel2(&nearest_op.unwrap(), voxel);
    }
    
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

      
      if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
        
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
        })
      }
    }
  }


}

fn update_by_range(
  mut ranges: Query<(&Range, &ChunkEdit, &mut Chunks)>,

  mut game_res: ResMut<GameResource>,

  hotbar_res: Res<HotbarResource>,
  mut mouse_inputs: EventReader<MouseInput>,
  mut physics: ResMut<Physics>,
) {
  let mut voxel_op = None;
  for event in mouse_inputs.iter() {
    if event.mouse_button_input.state == ButtonState::Pressed {
      if event.mouse_button_input.button == MouseButton::Left {
        voxel_op = Some(0);
      }
      
      if event.mouse_button_input.button == MouseButton::Right {
        voxel_op = Some(1);
        for i in 0..hotbar_res.bars.len() {
          let bar = &hotbar_res.bars[i];
          if  hotbar_res.selected_keycode ==  bar.key_code {
            voxel_op = Some(bar.voxel);
          }
        }
      }
    }
  }
  if voxel_op.is_none() {
    return;
  }

  let voxel = voxel_op.unwrap();

  for (range, chunk_edit, mut chunks) in &mut ranges {
    let size = 2_u32.pow(range.scale as u32);

    // info!("size {}", size_u32);

    let mut min = -(range.scale as i64);
    let mut max = (range.scale as i64);

    if chunk_edit.snap_mode == SnapMode::Grid {
      min = 0;
      max = size as i64;
    }
    

    if is_inside_chunk(min, max, range.point, &game_res.chunk_manager) {
      info!("is_inside_chunk");
      continue;
    }

    let mut res = HashMap::new();
    for x in min..max {
      for y in min..max {
        for z in min..max {

          let pos = [
            range.point.x as i64 + x,
            range.point.y as i64 + y,
            range.point.z as i64 + z
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
        &mut game_res.chunk_manager.voxel_reuse
      );

      
      if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
        
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


fn is_inside_chunk(
  min: i64,
  max: i64,
  point: Vec3,
  chunk_manager: &ChunkManager,
) -> bool {
  let buffered_min = min - 1;
  let buffered_max = max + 1;
  for x in buffered_min..buffered_max {
    for y in buffered_min..buffered_max {
      for z in buffered_min..buffered_max {
        let pos = [
          point.x as i64 + x,
          point.y as i64 + y,
          point.z as i64 + z
        ];

        let voxel = chunk_manager.get_voxel(&pos);
        if voxel == 0 {
          return false
        }
      }
    }
  }

  true
}

#[derive(Component)]
pub struct ChunkEdit {
  pub mode: EditMode,
  pub snap_mode: SnapMode
}

impl Default for ChunkEdit {
  fn default() -> Self {
    Self {
      mode: EditMode::Create,
      snap_mode: SnapMode::Grid,
    }
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum EditMode {
  Create,
  Delete,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum SnapMode {
  Normal,
  Grid,
}
use bevy::prelude::*;

use crate::{BevyVoxelResource, Preview, Chunks, MeshComponent};

mod add_normal;
mod add_dist;
mod add_snap;

mod remove_normal;
mod remove_dist;
mod remove_snap;

mod dist_common;
mod normal_common;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<EditEvents>()
      .add_plugins(add_normal::CustomPlugin)
      .add_plugins(add_dist::CustomPlugin)
      .add_plugins(add_snap::CustomPlugin)
      
      .add_plugins(remove_normal::CustomPlugin)
      .add_plugins(remove_dist::CustomPlugin)
      .add_plugins(remove_snap::CustomPlugin)

      .add_plugins(normal_common::CustomPlugin)
      .add_plugins(dist_common::CustomPlugin)
      .add_systems(Update, modify_voxels);
  }
}


fn modify_voxels(
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Preview, &mut Chunks, &mut MeshComponent)>,

  mut edit_event_reader: EventReader<EditEvents>,
) {
  for e in edit_event_reader.iter() {
    if e.event == EditEvent::RemoveCube {
      for (preview, mut chunks, mut mesh_comp) in &mut chunks {
        if preview.pos.is_none() {
          continue;
        }

        let p = preview.pos.unwrap();
        let res = bevy_voxel_res.set_voxel_cube_default(p, preview.size, 0);

        let mut all_chunks = Vec::new();
        for (key, chunk) in res.iter() {
          all_chunks.push(chunk.clone());
          chunks.data.insert(*key, chunk.clone());
        }

        let data = bevy_voxel_res.load_mesh_data(&all_chunks);
        for (mesh_data, handle) in data.iter() {
          
          mesh_comp.data.insert(mesh_data.key.clone(), mesh_data.clone());
          mesh_comp.added.push((mesh_data.clone(), *handle));
        }
      }
    }

    if e.event == EditEvent::RemoveSphere {
      for (preview, mut chunks, mut mesh_comp) in &mut chunks {
        if preview.pos.is_none() {
          continue;
        }

        let p = preview.pos.unwrap();
        let res = bevy_voxel_res.set_voxel_sphere_default(p, preview.sphere_size, 0);

        let mut all_chunks = Vec::new();
        for (key, chunk) in res.iter() {
          all_chunks.push(chunk.clone());
          chunks.data.insert(*key, chunk.clone());
        }

        let data = bevy_voxel_res.load_mesh_data(&all_chunks);
        for (mesh_data, handle) in data.iter() {
          mesh_comp.data.insert(mesh_data.key.clone(), mesh_data.clone());
          mesh_comp.added.push((mesh_data.clone(), *handle));
        }
      }
    }
  }
}



#[derive(Event)]
pub struct EditEvents {
  pub event: EditEvent
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum EditEvent {
  AddCube,
  AddSphere,
  RemoveCube,
  RemoveSphere,
}



use bevy::prelude::*;
use crate::{BevyVoxelResource, Chunks, Center, ChunkData};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    // app
    //   .add_system(added_chunks)
    //   .add_system(center_changed);
  }
}

fn added_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks), Added<Chunks>>
) {
  for (center, mut chunks) in &mut chunks {
    println!("added chunks");

    let lod = res.chunk_manager.depth;

    let mut meshes = res.load_lod_meshes(center.key, lod as u8 - 1);
    meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 2));
    for mesh in meshes.iter() {
      chunks.data.push(ChunkData {
        data: mesh.mesh.clone(),
        key: mesh.key,
      });
    }
  }
}

fn center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks), Changed<Center>>
) {
  for (center, mut chunks) in &mut centers {
    let lod = res.chunk_manager.depth;

    let mut meshes = res.load_lod_meshes(center.key, lod as u8 - 1);
    meshes.append(&mut res.load_lod_meshes(center.key, lod as u8 - 2));
    for mesh in meshes.iter() {
      chunks.data.push(ChunkData {
        data: mesh.mesh.clone(),
        key: mesh.key,
      });
    }
  }
}

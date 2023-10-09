use bevy::prelude::*;
use multithread::plugin::{PluginResource, send_key, Key, send_chunk};
use crate::BevyVoxelResource;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, (
        recv_keys,
        recv_chunk,
        recv_process_mesh,
        load_mesh
      ));
  }
}

fn recv_keys(
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (key, lod) in bevy_voxel_res.recv_key.drain() {
    let key = key.clone();
    send_key(Key {
      key: key,
      lod: lod
    });
  } 
}

fn recv_chunk(
  plugin_res: Res<PluginResource>,
) {
  for chunk in plugin_res.recv_chunk.drain() {
    send_chunk(chunk);
  }
  
}

fn recv_process_mesh(
  bevy_voxel_res: Res<BevyVoxelResource>,
) {

  for chunk in bevy_voxel_res.recv_process_mesh.drain() {
    send_chunk(chunk);
  }
  
}

fn load_mesh(
  plugin_res: Res<PluginResource>,
  bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for data in plugin_res.recv_mesh.drain() {
    let _ = bevy_voxel_res.send_mesh.send(data);
  }
}
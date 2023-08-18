use bevy::prelude::*;
use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "test_fast_surface_nets")] {
    mod chunk;
  }
}

cfg_if! {
  if #[cfg(feature = "test_mesh_color")] {
    mod mesh_color;
  }
}

cfg_if! {
  if #[cfg(feature = "test_voxel_size_config")] {
    mod voxel_size_config;
  }
}

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
  fn build(&self, app: &mut App) {
    cfg_if! {
      if #[cfg(feature = "test_mesh_color")] {
        app
          .add_plugin(mesh_color::CustomPlugin);
      }
    }

    cfg_if! {
      if #[cfg(feature = "test_voxel_size_config")] {
        app
          .add_plugin(voxel_size_config::CustomPlugin);
      }
    }
    
  }
}

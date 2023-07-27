use bevy::prelude::*;

mod chunk;

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
  fn build(&self, app: &mut App) {
    // app
    //   .add_plugin(chunk::CustomPlugin);
  }
}
use bevy::prelude::*;
use crate::components::player::Player;
use super::ChunkPreviewGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, remove);
  }
}

fn remove(
  mut commands: Commands,
  mut players: RemovedComponents<Player>,
  graphics: Query<(Entity, &ChunkPreviewGraphics)>,
) {
  for entity in &mut players {
    for (graphics_entity, graphics) in &graphics {
      if entity == graphics.parent {
        commands.entity(graphics_entity).despawn_recursive();
      }
    }
  }
  
}



#[derive(Component)]
pub struct ChunkPreviewRender {
  pub entities: Vec<Entity>,
}

impl Default for ChunkPreviewRender {
  fn default() -> Self {
    Self {
      entities: Vec::new(),
    }
  }
}

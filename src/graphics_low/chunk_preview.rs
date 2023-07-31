use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use voxels::data::voxel_octree::VoxelMode;
use crate::components::chunk_edit::{ChunkEditResource, EditMode};
use crate::components::player::Player;
use crate::graphics::chunk_preview::ChunkPreviewRender;
use crate::graphics::{GraphicsResource, ChunkPreviewGraphics};
use crate::{data::GameResource, components::chunk_preview::ChunkPreview};
use super::chunks::{VOXEL_WEIGHT, VOXEL_TYPE_1};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      // .add_system(hook_to_player)
      .add_system(update);
  }
}

fn hook_to_player(
  mut commands: Commands,
  players: Query<Entity, Added<Player>>,
) {
  for entity in &players {
    commands
      .entity(entity)
      .insert(ChunkPreviewRender::default());
  }
}


fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut game_res: ResMut<GameResource>,
  chunk_previews: Query<
    (Entity, &ChunkPreview), Changed<ChunkPreview>
  >,
  mut materials: ResMut<Assets<StandardMaterial>>,
  graphics_res: Res<GraphicsResource>,

  graphics: Query<(Entity, &ChunkPreviewGraphics)>,

  chunk_edit_res: Res<ChunkEditResource>,
) {
  for (entity, chunk_preview) in &chunk_previews {
    for (graphics_entity, graphics) in &graphics {
      if entity == graphics.parent {
        commands.entity(graphics_entity).despawn_recursive();
      }
    }

    if chunk_preview.chunk_op.is_none() {
      continue;
    }

    let chunk = chunk_preview.chunk_op.clone().unwrap();

    let data = chunk.octree.compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut game_res.chunk_manager.voxel_reuse
    );

    if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));
  
      render_mesh.insert_attribute(VOXEL_WEIGHT, data.weights.clone());
      render_mesh.insert_attribute(VOXEL_TYPE_1, data.types_1.clone());

      let chunk_size = (chunk.octree.get_size() / 2) as f32;
      let p = &chunk_preview.new;
      let adj = [p[0] as f32, p[1] as f32, p[2] as f32];
      let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];
      
      let mut visibility = Visibility::Visible;
      if !graphics_res.show_preview {
        visibility = Visibility::Hidden;
      }

      let mut color = Color::rgba(0.0, 0.0, 1.0, 0.25);
      if chunk_edit_res.edit_mode == EditMode::DeleteNormal ||
      chunk_edit_res.edit_mode == EditMode::DeleteSnap {
        color = Color::rgba(1.0, 0.0, 0.0, 0.25);
      }

      commands
        .spawn(MaterialMeshBundle {
          visibility: visibility,
          mesh: meshes.add(render_mesh),
          material: materials.add(color.into()),
          transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
          ..default()
        })
        .insert(ChunkPreviewGraphics { parent: entity });
    }
  }
}


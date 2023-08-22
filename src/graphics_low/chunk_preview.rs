use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use voxels::data::voxel_octree::VoxelMode;
use crate::components::chunk_edit::{ChunkEdit, EditState};
use crate::graphics::ChunkPreviewGraphics;
use crate::data::GameResource;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update);
  }
}

fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  game_res: Res<GameResource>,
  edits: Query<(Entity, &ChunkEdit), Changed<ChunkEdit>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

  edit_state: Res<State<EditState>>,
  graphics: Query<(Entity, &ChunkPreviewGraphics)>,
) {
  // for (entity, edit) in &edits {
  //   for (graphics_entity, graphics) in &graphics {
  //     if entity == graphics.parent {
  //       commands.entity(graphics_entity).despawn_recursive();
  //     }
  //   }

  //   if edit.chunk.is_none() {
  //     continue;
  //   }

  //   let chunk = edit.chunk.clone().unwrap();

  //   let data = chunk.octree.compute_mesh(
  //     VoxelMode::SurfaceNets, 
  //     &mut game_res.chunk_manager.voxel_reuse.clone(),
  //     &game_res.colors,
  //     game_res.voxel_scale,
  //   );

  //   if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
  //     let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  //     render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  //     render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  //     render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  //     let chunk_size = (chunk.octree.get_size() / 2) as f32 * game_res.voxel_scale;
  //     let p = &edit.position.unwrap();
  //     let adj = [p.x as f32, p.y as f32, p.z as f32];
  //     let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];
      
  //     let mut color = Color::rgba(0.0, 0.0, 1.0, 0.25);
  //     match edit_state.0 {
  //       EditState::RemoveNormal |
  //       EditState::RemoveSnap => {
  //         color = Color::rgba(1.0, 0.0, 0.0, 0.25);
  //       },
  //       _ => {}
  //     }

  //     commands
  //       .spawn(MaterialMeshBundle {
  //         // visibility: visibility,
  //         mesh: meshes.add(render_mesh),
  //         material: materials.add(color.into()),
  //         transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
  //         ..default()
  //       })
  //       .insert(ChunkPreviewGraphics { parent: entity });
  //   }
  // }
}


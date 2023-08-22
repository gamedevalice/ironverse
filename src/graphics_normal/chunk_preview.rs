use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_voxel::{BevyVoxelResource, Preview, PreviewGraphics, EditState};
use voxels::data::voxel_octree::VoxelMode;

use super::chunks::{CustomMaterial, VOXEL_COLOR};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(update.in_set(OnUpdate(EditState::AddNormal)));
  }
}

fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  previews: Query<&Preview, Changed<Preview>>,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,

  mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
  for preview in &previews {
    for entity in &preview_graphics {
      commands.entity(entity).despawn_recursive();
    }

    if preview.pos.is_none() {
      continue;
    }

    let p = preview.pos.unwrap();
    let chunk = bevy_voxel_res.get_preview_chunk(p, preview.voxel);
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, &chunk);

    if data.indices.len() > 0 {
      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

      let mesh_handle = meshes.add(render_mesh);
      let material_handle = custom_materials.add(CustomMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
      });

      let pos = bevy_voxel_res.get_preview_pos(p);
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: material_handle,
          transform: Transform::from_translation(pos),
          ..default()
        })
        .insert(PreviewGraphics);
    }
  }
}
/* 
fn update_remove(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  game_res: Res<GameResource>,
  edits: Query<(Entity, &ChunkEdit), Changed<ChunkEdit>>,
  graphics: Query<(Entity, &ChunkPreviewGraphics)>,

  mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
  for (entity, edit) in &edits {
    for (graphics_entity, graphics) in &graphics {
      if entity == graphics.parent {
        commands.entity(graphics_entity).despawn_recursive();
      }
    }

    if edit.chunk.is_none() {
      continue;
    }

    let chunk = edit.chunk.clone().unwrap();

    let data = chunk.octree.compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut game_res.chunk_manager.voxel_reuse.clone(),
      &game_res.colors,
      game_res.voxel_scale,
    );

    if data.indices.len() > 0 { // Temporary, should be removed once the ChunkMode detection is working
      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

      let mesh_handle = meshes.add(render_mesh);
      let material_handle = custom_materials.add(CustomMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
      });


      let chunk_size = (chunk.octree.get_size() / 2) as f32;
      let p = &edit.position.unwrap();
      let adj = [p.x as f32, p.y as f32, p.z as f32];
      let coord_f32 = [adj[0] - chunk_size, adj[1] - chunk_size, adj[2] - chunk_size];

      commands
        .spawn(MaterialMeshBundle {
          // visibility: visibility,
          mesh: mesh_handle,
          material: material_handle,
          transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
          ..default()
        })
        .insert(ChunkPreviewGraphics { parent: entity });
    }
  }
}
 */

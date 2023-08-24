use bevy::{prelude::*, pbr::NotShadowCaster};
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_voxel::{BevyVoxelResource, Preview, PreviewGraphics, EditState};
use voxels::data::voxel_octree::VoxelMode;

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
  mut materials: ResMut<Assets<StandardMaterial>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  previews: Query<&Preview, Changed<Preview>>,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,
) {
  for preview in &previews {
    for entity in &preview_graphics {
      commands.entity(entity).despawn_recursive();
    }

    if preview.pos.is_none() {
      continue;
    }

    let p = preview.pos.unwrap();
    let chunk = bevy_voxel_res.get_preview(p, preview);
    
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, &chunk);
    let pos = bevy_voxel_res.get_preview_pos(p);

    let mut render = Mesh::new(PrimitiveTopology::TriangleList);
    render.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render.set_indices(Some(Indices::U32(data.indices.clone())));

    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render),
        material: materials.add(Color::rgba(0.7, 0.7, 0.7, 0.5).into()),
        transform: Transform::from_translation(pos),
        ..default()
      })
      .insert(PreviewGraphics)
      .insert(NotShadowCaster);
  }
}
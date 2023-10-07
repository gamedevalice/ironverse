use bevy::{prelude::*, pbr::NotShadowCaster};
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_voxel::{BevyVoxelResource, Preview, PreviewGraphics, EditState};
use voxels::data::voxel_octree::VoxelMode;

use super::chunks::{CustomMaterial, VOXEL_COLOR};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update.run_if(edit_add))
      .add_systems(Update, update_remove.run_if(edit_remove));
  }
}

fn edit_add(edit_state: Res<State<EditState>>,) -> bool {
  *State::get(&edit_state) == EditState::AddNormal ||
  *State::get(&edit_state) == EditState::AddDist ||
  *State::get(&edit_state) == EditState::AddSnap
}

fn edit_remove(edit_state: Res<State<EditState>>,) -> bool {
  *State::get(&edit_state) == EditState::RemoveNormal ||
  *State::get(&edit_state) == EditState::RemoveDist ||
  *State::get(&edit_state) == EditState::RemoveSnap
}


fn update(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
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
    let chunk = bevy_voxel_res.get_preview(p, preview);
    
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, &chunk);
    let pos = bevy_voxel_res.get_preview_pos(p);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));
    render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

    let mesh_handle = meshes.add(render_mesh);
    let material_handle = custom_materials.add(CustomMaterial {
      base_color: Color::rgb(1.0, 1.0, 1.0),
    });

    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform::from_translation(pos),
        ..default()
      })
      .insert(PreviewGraphics)
      .insert(NotShadowCaster);
  }
}

fn update_remove(
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
        material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.3).into()),
        transform: Transform::from_translation(pos),
        ..default()
      })
      .insert(PreviewGraphics)
      .insert(NotShadowCaster);
  }
}


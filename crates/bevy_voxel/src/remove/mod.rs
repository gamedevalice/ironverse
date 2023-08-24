use bevy::{prelude::*, pbr::NotShadowCaster};
use voxels::data::voxel_octree::VoxelMode;
use crate::{BevyVoxelResource, EditState, Chunks, Center, ChunkData, Selected, SelectedGraphics};

mod bydist;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(bydist::CustomPlugin)
      .add_systems(
        (remove_voxel, reposition_selected_voxel)
          .in_set(OnUpdate(EditState::RemoveNormal)))
      .add_system(remove.in_schedule(OnExit(EditState::RemoveNormal)));
  }
}


fn remove_voxel(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Selected, &Center, &mut Chunks)>,
) {
  let mut voxel = None;
  
  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(0);
  }
  if voxel.is_none() {
    return;
  }

  for (selected, center, mut chunks) in &mut chunks {
    if selected.pos.is_none() {
      continue;
    }

    
    chunks.data.clear();
    
    let p = selected.pos.unwrap();
    bevy_voxel_res.set_voxel(p, voxel.unwrap());

    let all_chunks = bevy_voxel_res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }
}

fn reposition_selected_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  selecteds: Query<&Selected, Changed<Selected>>,
  selected_graphics: Query<Entity, With<SelectedGraphics>>,
) {
  for selected in &selecteds {
    for entity in &selected_graphics {
      commands.entity(entity).despawn_recursive();
    }

    if selected.pos.is_none() {
      continue;
    }

    let p = selected.pos.unwrap();
    let scale = bevy_voxel_res.chunk_manager.voxel_scale;
    let size = scale + (scale * 0.1);
    commands.spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: size})),
      material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.5).into()),
      transform: Transform::from_translation(p),
      ..default()
    })
    .insert(SelectedGraphics)
    .insert(NotShadowCaster);

  }
}

fn remove(
  mut commands: Commands,
  selected_graphics: Query<Entity, With<SelectedGraphics>>,
) {
  for entity in &selected_graphics {
    commands.entity(entity).despawn_recursive();
  }
}


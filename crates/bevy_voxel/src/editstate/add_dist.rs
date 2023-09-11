use bevy::prelude::*;
use voxels::data::voxel_octree::VoxelMode;
use crate::{EditState, Preview, BevyVoxelResource, PreviewGraphics, Chunks, Center, ChunkData, ShapeState, MeshComponent};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add_voxel_cube.in_set(OnUpdate(EditState::AddDist)))
      .add_system(add_voxel_sphere.in_set(OnUpdate(EditState::AddDist)))
      .add_system(remove.in_schedule(OnExit(EditState::AddDist)));
  }
}

fn add_voxel_cube(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks, &mut MeshComponent)>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Cube {
    return;
  }
  for (preview, center, mut chunks, mut mesh_comp) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    let p = preview.pos.unwrap();
    let res = bevy_voxel_res.set_voxel_cube(p, preview);

    let mut all_chunks = Vec::new();
    for (key, chunk) in res.iter() {
      all_chunks.push(chunk.clone());
      chunks.data.insert(*key, chunk.clone());
    }

    let data = bevy_voxel_res.load_mesh_data(&all_chunks);
    for (mesh_data, handle) in data.iter() {
      
      mesh_comp.data.insert(mesh_data.key.clone(), mesh_data.clone());
      mesh_comp.added.push((mesh_data.clone(), *handle));
    }
  }
}

fn add_voxel_sphere(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks, &mut MeshComponent)>,
  shape_state: Res<State<ShapeState>>,
) {
  if !mouse.just_pressed(MouseButton::Left) ||
  shape_state.0 != ShapeState::Sphere {
    return;
  }

  for (preview, center, mut chunks, mut mesh_comp) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    let p = preview.pos.unwrap();
    let res = bevy_voxel_res.set_voxel_sphere(p, preview);

    let mut all_chunks = Vec::new();
    for (key, chunk) in res.iter() {
      all_chunks.push(chunk.clone());
      chunks.data.insert(*key, chunk.clone());
    }

    let data = bevy_voxel_res.load_mesh_data(&all_chunks);
    for (mesh_data, handle) in data.iter() {
      mesh_comp.data.insert(mesh_data.key.clone(), mesh_data.clone());
      mesh_comp.added.push((mesh_data.clone(), *handle));
    }
  }
}


fn remove(
  mut commands: Commands,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,
) {
  for entity in &preview_graphics {
    commands.entity(entity).despawn_recursive();
  }
}

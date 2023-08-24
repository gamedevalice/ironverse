use bevy::{prelude::*, pbr::NotShadowCaster};
use crate::{EditState, Preview, BevyVoxelResource, Chunks, Center, SelectedGraphics, Selected, ChunkData};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        (remove_voxel, reposition_selected_voxel)
          .in_set(OnUpdate(EditState::RemoveNormal)))
      .add_system(remove.in_schedule(OnExit(EditState::RemoveNormal)));
  }
}


fn remove_voxel(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
) {
  if !mouse.just_pressed(MouseButton::Left) {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }
    chunks.data.clear();
    bevy_voxel_res.set_voxel(preview.pos.unwrap(), 1);

    let res = bevy_voxel_res.load_adj_mesh_data(center.key);
    for (key, data) in res.iter() {
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: *key,
      });
    }
  }
}


















/// DEBUGGER: Remove Visibility::Hidden to see what voxel is hit
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
      visibility: Visibility::Hidden, 
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


/*
  All logic of RemoveNormal should be managed here
    It should be presented by just a function
    Wrapped in BevyVoxelResource
  BevyVoxelResource will have to wrapped it

*/


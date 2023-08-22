use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, pbr::NotShadowCaster};
use voxels::data::voxel_octree::VoxelMode;
use crate::{BevyVoxelResource, EditState, Chunks, Center, ChunkData, Selected, Preview, PreviewGraphics};


pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(
        (reposition_preview_voxel, add_voxel).in_set(OnUpdate(EditState::AddNormal))
      );
  }
}


fn add_voxel(
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  mut chunks: Query<(&Preview, &Center, &mut Chunks)>,
) {
  let mut voxel = None;
  
  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(1);
  }
  if voxel.is_none() {
    return;
  }

  for (preview, center, mut chunks) in &mut chunks {
    if preview.pos.is_none() {
      continue;
    }

    chunks.data.clear();
    let p = preview.pos.unwrap();
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

fn reposition_preview_voxel(
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
    // println!("preview {:?}", p);
    let chunk = bevy_voxel_res.get_preview_chunk(p);
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
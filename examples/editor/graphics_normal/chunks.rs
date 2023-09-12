use bevy::{prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayout, Indices}, render_resource::{VertexFormat, AsBindGroup, ShaderRef, SpecializedMeshPipelineError, RenderPipelineDescriptor, PrimitiveTopology, ShaderType, AsBindGroupShaderType, TextureFormat}, render_asset::RenderAssets}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey, StandardMaterialFlags}, asset::LoadState};
use bevy_voxel::{BevyVoxelResource, Chunks, MeshComponent, Center};

use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(MaterialPlugin::<CustomMaterial>::default())
      .add_system(add)
      .add_system(remove);
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut images: ResMut<Assets<Image>>,

  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  mut chunk_query: Query<(Entity, &mut MeshComponent), Changed<MeshComponent>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for (_, mut mesh_comp) in &mut chunk_query {
    for (data, collider_handle) in mesh_comp.added.iter() {
      'graphics: for (entity, graphics) in &chunk_graphics {
        if graphics.key == data.key {
          commands.entity(entity).despawn();

          if graphics.lod == 0 {
            bevy_voxel_res.physics.remove_collider(graphics.collider);
          }
          continue 'graphics;
        }
      }

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));
      render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

      let mesh_handle = meshes.add(render_mesh);
      let material_handle = custom_materials.add(CustomMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
      });

      let mut pos = bevy_voxel_res.get_pos(data.key);
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: material_handle,
          transform: Transform::from_translation(pos),
          ..default()
        })
        .insert(ChunkGraphics { 
          key: data.key, 
          lod: data.lod as usize,
          collider: *collider_handle,
        });

      // println!("data.lod {}", data.lod);
    }
    mesh_comp.added.clear();
  }


  // for (_, chunks) in &chunk_query {
  //   for (entity, graphics) in &chunk_graphics {
  //     commands.entity(entity).despawn_recursive();
  //   }

  //   for mesh in &chunks.data {

  //     let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  //     render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh.positions.clone());
  //     render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh.normals.clone());
  //     render_mesh.set_indices(Some(Indices::U32(mesh.indices.clone())));
  //     render_mesh.insert_attribute(VOXEL_COLOR, mesh.colors.clone());

  //     let mesh_handle = meshes.add(render_mesh);
  //     let material_handle = custom_materials.add(CustomMaterial {
  //       base_color: Color::rgb(1.0, 1.0, 1.0),
  //     });

  //     let mut pos = bevy_voxel_res.get_pos(mesh.key);
  //     commands
  //       .spawn(MaterialMeshBundle {
  //         mesh: mesh_handle,
  //         material: material_handle,
  //         transform: Transform::from_translation(pos),
  //         ..default()
  //       })
  //       .insert(ChunkGraphics);
  //   }
  // }
}

fn remove(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Center)>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {

  let ranges = bevy_voxel_res.ranges.clone();
  for (_, center) in &chunk_query {
    for (entity, graphics) in &chunk_graphics {

      if !bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, graphics.lod) {
        commands.entity(entity).despawn_recursive();

        if graphics.lod == 0 {
          bevy_voxel_res.physics.remove_collider(graphics.collider);
          // println!("remove collider 2");
        }
      }
      
    }
  }
}


pub const VOXEL_COLOR: MeshVertexAttribute =
  MeshVertexAttribute::new("VOXEL_COLOR", 988540918, VertexFormat::Float32x3);

#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[uuid = "2f3d7f74-4bf7-4f32-98cd-858edafa5ca2"]
pub struct CustomMaterial {
  pub base_color: Color,
}

impl Material for CustomMaterial {
  fn vertex_shader() -> ShaderRef {
    "shaders/color_vertex.wgsl".into()
  }
  fn fragment_shader() -> ShaderRef {
    "shaders/color_fragment.wgsl".into()
  }
  fn specialize(
    _pipeline: &MaterialPipeline<Self>,
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayout,
    _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout.get_layout(&[
      Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
      Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
      VOXEL_COLOR.at_shader_location(2),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];

    Ok(())
  }
}










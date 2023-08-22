use bevy::{prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayout, Indices}, render_resource::{VertexFormat, AsBindGroup, ShaderRef, SpecializedMeshPipelineError, RenderPipelineDescriptor, PrimitiveTopology, ShaderType, AsBindGroupShaderType, TextureFormat}, render_asset::RenderAssets}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey, StandardMaterialFlags}, asset::LoadState};
use bevy_voxel::{BevyVoxelResource, Chunks};

use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(MaterialPlugin::<CustomMaterial>::default())
      .add_system(add);
  }
}

fn add(
  bevy_voxel_res: Res<BevyVoxelResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut images: ResMut<Assets<Image>>,

  chunk_query: Query<(Entity, &Chunks), Changed<Chunks>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,
) {
  for (_, chunks) in &chunk_query {
    println!("Changed");
    for (entity, graphics) in &chunk_graphics {
      commands.entity(entity).despawn_recursive();
    }

    for mesh in &chunks.data {
      let data = &mesh.data;

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));
      render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

      let mesh_handle = meshes.add(render_mesh);
      let material_handle = custom_materials.add(CustomMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
      });

      let mut pos = bevy_voxel_res.get_pos(mesh.key);
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: material_handle,
          transform: Transform::from_translation(pos),
          ..default()
        })
        .insert(ChunkGraphics);
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










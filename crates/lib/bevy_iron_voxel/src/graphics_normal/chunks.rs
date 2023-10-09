use bevy::{prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayout, Indices}, render_resource::{VertexFormat, AsBindGroup, ShaderRef, SpecializedMeshPipelineError, RenderPipelineDescriptor, PrimitiveTopology}}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey}};
use bevy_voxel::{BevyVoxelResource, MeshComponent};
use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(MaterialPlugin::<CustomMaterial>::default())
      .add_systems(Update, add);


/*     // Test code
    app
      .add_systems(Update, 
        delete_main_octrees_outside_range
      ); */
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut _images: ResMut<Assets<Image>>,

  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  mut chunk_query: Query<(Entity, &mut MeshComponent), Changed<MeshComponent>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for (_, mut mesh_comp) in &mut chunk_query {
    for (data, collider_handle) in mesh_comp.added.iter() {

      // This is for removing the duplicates
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

      let pos = bevy_voxel_res.get_pos(data.key);
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
}

/* fn delete_main_octrees_outside_range(
  mut commands: Commands,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
  centers: Query<&Center, Changed<Center>>,

  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

) {
  for center in &centers {
    for (entity, graphics) in &chunk_graphics {
      if !bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, 0) {
        if graphics.lod == 0 {
          commands.entity(entity).despawn();
          bevy_voxel_res.physics.remove_collider(graphics.collider);
        }
        
      }
    }
    
  }
} */




pub const VOXEL_COLOR: MeshVertexAttribute =
  MeshVertexAttribute::new("VOXEL_COLOR", 988540918, VertexFormat::Float32x3);

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
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










use bevy::{prelude::*, render::{mesh::{MeshVertexAttribute, MeshVertexBufferLayout, Indices}, render_resource::{VertexFormat, AsBindGroup, ShaderRef, SpecializedMeshPipelineError, RenderPipelineDescriptor, PrimitiveTopology, ShaderType, AsBindGroupShaderType, TextureFormat}, render_asset::RenderAssets}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey, StandardMaterialFlags}, asset::LoadState};
use voxels::{utils::{key_to_world_coord_f32}, data::voxel_octree::{VoxelMode, MeshData}, chunk::adjacent_keys};

use crate::{graphics::ChunkGraphics, components::{chunk::Chunks, player::Player}, data::GameResource};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_plugin(MaterialPlugin::<CustomMaterial>::default())
      .add_startup_system(startup)
      .add_system(add)
      .add_system(remove);
  }
}


fn startup(
  mut commands: Commands, 
  asset_server: Res<AssetServer>,
) {
  // commands.spawn(PointLightBundle {
  //   point_light: PointLight {
  //     intensity: 3000.0,
  //     ..Default::default()
  //   },
  //   transform: Transform::from_xyz(-3.0, 2.0, -1.0),
  //   ..Default::default()
  // });
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 3000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(0.0, 5.0, 0.0),
    ..Default::default()
  });
}

fn add(
  mut game_res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut _materials: ResMut<Assets<StandardMaterial>>,
  mut images: ResMut<Assets<Image>>,
  terrains: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Chunks), Changed<Chunks>>,
) {
  for (_, chunks) in &chunk_query {
    for mesh in &chunks.data {
      'inner: for (entity, terrain) in &terrains {
        if mesh.key == terrain.key {
          commands.entity(entity).despawn_recursive();
          break 'inner;
        }
      }

      if mesh.data.positions.len() > 0 {
        let mut queue = true;
        for (key, _) in local_res.queued_chunks.iter() {
          if key == &mesh.key {
            queue = false;
          }
        }
        
        if queue {
          local_res.queued_chunks.push((mesh.key.clone(), mesh.data.clone()));
        }
      }
      
    }
    
  }
  
  let config = game_res.chunk_manager.config.clone();
  for (key, data) in local_res.queued_chunks.iter() {
    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());

    let mesh_handle = meshes.add(render_mesh);
    let material_handle = custom_materials.add(CustomMaterial {
      base_color: Color::rgb(1.0, 1.0, 1.0),
    });

    let coord_f32 = key_to_world_coord_f32(key, config.seamless_size);
    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
        ..default()
      })
      .insert(ChunkGraphics { key: *key });
  }

  local_res.queued_chunks.clear();
}

fn remove(
  mut commands: Commands,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Chunks, &Player), Changed<Chunks>>,
) {
  for (_, _chunks, player) in &chunk_query {
    let adj_keys = adjacent_keys(&player.key, 1, true);
    for (entity, graphics) in &chunk_graphics {
      if !adj_keys.contains(&graphics.key) {
        commands.entity(entity).despawn_recursive();
      }
    }
  
    
  }
}

#[derive(Resource)]
struct LocalResource {
  queued_chunks: Vec<([i64; 3], MeshData)>,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      queued_chunks: Vec::new(),
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










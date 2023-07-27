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
      .add_system(init_textures)
      .add_system(add)
      .add_system(remove);
  }
}


fn startup(
  mut commands: Commands, 
  asset_server: Res<AssetServer>,
) {
  commands.insert_resource(ChunkTexture {
    is_loaded: false,
    // albedo: asset_server.load("textures/array_texture.png"),
    albedo: asset_server.load("textures/textures.png"),
    normal: asset_server.load("textures/textures_normals.png"),
  });

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

fn init_textures(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut _materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,

  mut chunk_texture: ResMut<ChunkTexture>,
  local_res: Res<LocalResource>,
  mut images: ResMut<Assets<Image>>,
) {
  if chunk_texture.is_loaded
    || asset_server.get_load_state(chunk_texture.albedo.clone()) != LoadState::Loaded
    || asset_server.get_load_state(chunk_texture.normal.clone()) != LoadState::Loaded
  {
    return;
  }
  chunk_texture.is_loaded = true;

  let array_layers = 16;
  let image = images.get_mut(&chunk_texture.albedo).unwrap();
  image.reinterpret_stacked_2d_as_array(array_layers);

  let normal = images.get_mut(&chunk_texture.normal).unwrap();
  normal.reinterpret_stacked_2d_as_array(array_layers);
}

fn add(
  mut game_res: ResMut<GameResource>,
  mut local_res: ResMut<LocalResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
  mut _materials: ResMut<Assets<StandardMaterial>>,
  mut chunk_texture: ResMut<ChunkTexture>,
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

  if !chunk_texture.is_loaded {
    return;
  }

  let config = game_res.chunk_manager.config.clone();
  for (key, data) in local_res.queued_chunks.iter() {
    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    render_mesh.insert_attribute(VOXEL_WEIGHT, data.weights.clone());
    render_mesh.insert_attribute(VOXEL_TYPE_1, data.types_1.clone());

    let mesh_handle = meshes.add(render_mesh);
    let material_handle = custom_materials.add(CustomMaterial {
      base_color: Color::rgb(1.0, 1.0, 1.0),
      albedo: chunk_texture.albedo.clone(),
      normal: chunk_texture.normal.clone(),
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


#[derive(Resource)]
pub struct ChunkTexture {
  pub is_loaded: bool,
  pub albedo: Handle<Image>,
  pub normal: Handle<Image>,
}


pub const VOXEL_WEIGHT: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Weight", 988540917, VertexFormat::Float32x4);

pub const VOXEL_TYPE_1: MeshVertexAttribute =
  MeshVertexAttribute::new("Voxel_Type_1", 988540918, VertexFormat::Uint32x4);



#[derive(AsBindGroup, Reflect, FromReflect, Debug, Clone, TypeUuid)]
#[uuid = "2f3d7f74-4bf7-4f32-98cd-858edafa5ca2"]
#[bind_group_data(TriplanarMaterialKey)]
#[uniform(0, TriplanarMaterialUniform)]
pub struct CustomMaterial {
  pub base_color: Color,

  #[texture(1, dimension = "2d_array")]
  #[sampler(2)]
  pub albedo: Handle<Image>,
  #[texture(3, dimension = "2d_array")]
  #[sampler(4)]
  pub normal: Handle<Image>,
}

impl Material for CustomMaterial {
  fn vertex_shader() -> ShaderRef {
    "shaders/triplanar_vertex.wgsl".into()
  }
  fn fragment_shader() -> ShaderRef {
    "shaders/triplanar_fragment.wgsl".into()
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
      VOXEL_WEIGHT.at_shader_location(2),
      VOXEL_TYPE_1.at_shader_location(3),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];

    Ok(())
  }

  // fn alpha_mode(&self) -> AlphaMode {
  //   AlphaMode::Blend
  // }
}

/// The GPU representation of the uniform data of a [`TriplanarMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct TriplanarMaterialUniform {
  pub base_color: Vec4,
  pub flags: u32,
}

impl AsBindGroupShaderType<TriplanarMaterialUniform> for CustomMaterial {
  fn as_bind_group_shader_type(&self, images: &RenderAssets<Image>) -> TriplanarMaterialUniform {
    let mut flags = StandardMaterialFlags::NONE;
    flags |= StandardMaterialFlags::BASE_COLOR_TEXTURE;

    if let Some(texture) = images.get(&self.normal) {
      match texture.texture_format {
        // All 2-component unorm formats
        TextureFormat::Rg8Unorm
        | TextureFormat::Rg16Unorm
        | TextureFormat::Bc5RgUnorm
        | TextureFormat::EacRg11Unorm => {
            flags |= StandardMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
        }
        _ => {}
      }
    }
    TriplanarMaterialUniform {
      base_color: self.base_color.as_linear_rgba_f32().into(),
      flags: flags.bits(),
    }
  }
}


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TriplanarMaterialKey {
  normal_map: bool,
}

impl From<&CustomMaterial> for TriplanarMaterialKey {
  fn from(material: &CustomMaterial) -> Self {
    TriplanarMaterialKey {
      normal_map: true,
    }
  }
}









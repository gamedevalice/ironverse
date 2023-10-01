use bevy::{prelude::*, render::{render_resource::{PrimitiveTopology, VertexFormat, AsBindGroup, RawRenderPipelineDescriptor, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError}, mesh::{Indices, MeshVertexAttribute, MeshVertexBufferLayout}}, window::{PrimaryWindow, CursorGrabMode}, reflect::TypeUuid, pbr::{MaterialPipeline, MaterialPipelineKey}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use voxels::{chunk::chunk_manager::{ChunkManager, Chunk}, utils::key_to_world_coord_f32, data::voxel_octree::{VoxelMode, VoxelOctree, MeshData}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(EguiPlugin)
      .add_plugin(MaterialPlugin::<CustomMaterial>::default())
      .add_startup_system(setup_camera)
      .add_startup_system(startup)
      .add_system(show_diagnostic_texts);
  }
}

fn setup_camera(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

  mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(6.5, 22.22, -8.4)
        .looking_to(Vec3::new(-0.0, -0.5, 0.8), Vec3::Y),
      // transform: Transform::from_xyz(2.8, 1.9, -0.5)
      //   .looking_to(Vec3::new(-0.1, -0.0, 0.98), Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam);

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 6000.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(0.0, 8.0, 8.0),
    ..Default::default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });


  let mut window = match windows.get_single_mut() {
    Ok(w) => { w },
    Err(_e) => return,
  };

  window.cursor.grab_mode = CursorGrabMode::None;
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>,
  mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
  let colors = vec![
    [1.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0], 
    [0.0, 0.0, 1.0], 
    [0.0, 0.0, 0.0],

    [0.2, 0.0, 0.0],
    [0.4, 0.0, 0.0],
    [0.6, 0.0, 0.0],
    [0.8, 0.0, 0.0],

    [0.0, 0.2, 0.0],
    [0.0, 0.4, 0.0],
  ];

  let manager = ChunkManager::default();
  let config = manager.config.clone();
  let chunk = ChunkManager::new_chunk(&[0, -1, 0], config.depth, config.lod, config.noise);


  // let mut chunk = Chunk::default();
  // chunk.octree.set_voxel(2, 2, 2, 1);
  // chunk.octree.set_voxel(3, 2, 2, 1);
  // chunk.octree.set_voxel(4, 2, 2, 1);
  // chunk.octree.set_voxel(5, 2, 2, 4);
  // chunk.octree.set_voxel(6, 2, 2, 5);
  // chunk.octree.set_voxel(7, 2, 2, 6);
  // chunk.octree.set_voxel(8, 2, 2, 7);
  // chunk.octree.set_voxel(9, 2, 2, 8);

  let data = chunk
    .octree
    .compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut manager.voxel_reuse.clone(),
      &colors,
    );

  // let data = get_data();
  // for i in 0..data.positions.len() {
  //   println!("{:?}", data.colors[i]);
  // }

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.insert_attribute(VOXEL_COLOR, data.colors.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  let mesh_handle = meshes.add(render_mesh);
  let material_handle = custom_materials.add(CustomMaterial {
    base_color: Color::rgb(1.0, 1.0, 1.0),
  });

  let coord_f32 = key_to_world_coord_f32(&[0, 0, 0], manager.config.seamless_size);
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: material_handle,
      // transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
      transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    });
}

fn show_diagnostic_texts(
  cameras: Query<&Transform, With<FlyCam>>,
  mut windows: Query<&mut Window, With<PrimaryWindow>>,
  key_input: Res<Input<KeyCode>>,

  mut ctx: EguiContexts,
) {
  let mut window = match windows.get_single_mut() {
    Ok(w) => { w },
    Err(_e) => return,
  };

  if key_input.just_pressed(KeyCode::LControl) {
    match window.cursor.grab_mode {
      CursorGrabMode::None => {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
      }
      _ => {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
      }
    }
  }
  

  let frame = Frame {
    fill: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
    ..Default::default()
  };

  bevy_egui::egui::Window::new("ChunkTexts")
    .title_bar(false)
    .frame(frame)
    .fixed_rect(Rect::from_min_max(
      Pos2::new(0.0, 0.0),
      Pos2::new(window.width(), window.height())
    ))
    .show(ctx.ctx_mut(), |ui| {
      ui.vertical(|ui| {
        let mut style = Style::default();
        style.spacing.item_spacing = Vec2::new(0.0, 0.0);
        ui.set_style(style);

        for trans in &cameras {
          ui.label(
            RichText::new(format!("Pos: {:?}", trans.translation))
              .color(Color32::WHITE)
              .size(20.0)
          );

          ui.label(
            RichText::new(format!("Forward: {:?}", trans.forward()))
              .color(Color32::WHITE)
              .size(20.0)
          );
        }
      });
    });

  
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


fn get_data() -> MeshData {
  let mut data = MeshData::default();

  /*
    Set the positions
    Normals
    Colors

    Set values between
    2.0, 1.0, 0.0

   */
  data.positions = vec![
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],

    [1.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],

    [2.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],

    [2.0, 1.0, 0.0],
    [2.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
  ];

  data.normals = vec![
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],

    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],

    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],

    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
    [0.0, 0.0, -1.0],
  ];

  data.colors = vec![
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.5, 0.0, 0.5],

    [0.5, 0.0, 0.5],
    [0.5, 0.0, 0.5],
    [1.0, 0.0, 0.0],

    [0.0, 0.0, 1.0],
    [0.5, 0.0, 0.5],
    [0.5, 0.0, 0.5],

    [0.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.5, 0.0, 0.5],
  ];

  data.indices = vec![
    0, 1, 2,
    3, 4, 5,

    6, 7, 8,
    9,10,11,
  ];


  println!("test");
  data
}

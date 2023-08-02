use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use voxels::{chunk::chunk_manager::ChunkManager, utils::key_to_world_coord_f32, data::voxel_octree::VoxelMode};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(EguiPlugin)
      .add_startup_system(startup)
      .add_startup_system(setup_camera)
      .add_system(show_diagnostic_texts);
  }
}

fn setup_camera(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.0, 5.0, -12.0)
        .looking_to(Vec3::new(0.0, -0.2, 0.97), Vec3::Y),
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
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mut manager = ChunkManager::default();

  let mut chunk = manager.new_chunk3(&[0, -1, 0], manager.config.lod);
  // chunk.octree.set_voxel(4, 13, 11, 0);
  chunk.octree.set_voxel(4, 13, 12, 0);

  let data = chunk
    .octree
    .compute_mesh(VoxelMode::SurfaceNets, &mut manager.voxel_reuse);

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  let mesh_handle = meshes.add(render_mesh);

  let coord_f32 = key_to_world_coord_f32(&[0, 0, 0], manager.config.seamless_size);
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
      transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
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



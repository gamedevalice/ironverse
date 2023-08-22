use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PresentMode, PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use bevy_voxel::BevyVoxelResource;
use voxels::{chunk::{chunk_manager::ChunkManager, adjacent_keys}, utils::key_to_world_coord_f32, data::{voxel_octree::VoxelMode, surface_nets::VoxelReuse}};
use bevy_flycam::NoCameraAndGrabPlugin;

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Ironverse Editor".into(),
        resolution: (800., 600.).into(),
        present_mode: PresentMode::AutoVsync,
        fit_canvas_to_parent: true,
        prevent_default_event_handling: false,
        ..default()
      }),
      ..default()
    }))
    .insert_resource(LocalResource::default())
    .add_plugin(EguiPlugin)
    .add_plugin(NoCameraAndGrabPlugin)
    .add_startup_system(setup_camera)
    .add_startup_system(startup_upper_left)
    // .add_startup_system(startup_upper_mid)
    .add_startup_system(startup_lower_left)
    .add_system(show_diagnostic_texts)
    .run();

}

fn setup_camera(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(Camera3dBundle {
      // transform: Transform::from_xyz(-5.89, 201.94, -59.06)
      transform: Transform::from_xyz(8.0, 33.0, 9.0)
        .looking_to(Vec3::new(0.0, -0.99, 0.07), Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam);

  // Sun
  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
        color: Color::rgb(0.98, 0.95, 0.82),
        shadows_enabled: true,
        illuminance: 10000.0,
        ..default()
    },
    transform: Transform::from_xyz(0.0, 50.0, 0.0)
        .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ..default()
  });

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 6000.0,
      ..Default::default()
    },
    // transform: Transform::from_xyz(6.0, 30.0, 6.0),
    transform: Transform::from_xyz(6.0, 15.0, 6.0),
    ..Default::default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(1.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
}

fn startup_upper_left(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  local_res: Res<LocalResource>,
) {
  let depth = 4;
  let voxel_scale = 1.0;
  let range = 1;

  let mut bevy_voxel_res = BevyVoxelResource::new(
    depth, 
    voxel_scale, 
    range,
    local_res.colors.clone(),
  );

  let key = [0, 0, 0];
  let pos_adj = Vec3::new(0.0, 0.0, 5.0);
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
    if !vec![[0, -1, 0]].contains(&chunk.key) {
      // continue;
    }
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
    let pos = bevy_voxel_res.get_pos(chunk.key);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render_mesh),
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_translation(pos + pos_adj),
        ..default()
      })
      .insert(ChunkGraphics {}); 
  }
}

fn startup_upper_mid(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  local_res: Res<LocalResource>,
) {
  let depth = 4;
  let voxel_scale = 0.5;
  let range = 1;

  let mut bevy_voxel_res = BevyVoxelResource::new(
    depth, 
    voxel_scale, 
    range,
    local_res.colors.clone(),
  );

  let key = [0, 0, 0];
  let pos_adj = Vec3::new(0.0, 0.0, 0.0);
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
    if !vec![[0, -1, 0]].contains(&chunk.key) {
      // continue;
    }

    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
    let pos = bevy_voxel_res.get_pos(chunk.key);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render_mesh),
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_translation(pos + pos_adj),
        ..default()
      })
      .insert(ChunkGraphics {}); 
  }
}

fn startup_lower_left(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  local_res: Res<LocalResource>,
) {
  let depth = 4;
  let voxel_scale = 0.25;
  let range = 4;

  let mut bevy_voxel_res = BevyVoxelResource::new(
    depth, 
    voxel_scale, 
    range,
    local_res.colors.clone(),
  );

  let key = [0, 0, 0];
  let pos_adj = Vec3::new(0.0, 5.0, 20.0);
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
    if !vec![[0, -1, 0]].contains(&chunk.key) {
      // continue;
    }

    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
    let pos = bevy_voxel_res.get_pos(chunk.key);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render_mesh),
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_translation(pos + pos_adj),
        ..default()
      })
      .insert(ChunkGraphics {}); 
  }
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


#[derive(Resource)]
struct LocalResource {
  colors: Vec<[f32; 3]>,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      colors: vec![
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
      ],
    }
  }
}

#[derive(Component, Clone)]
struct ChunkGraphics { }

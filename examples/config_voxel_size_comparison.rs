use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PresentMode, PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adj_keys_by_scale}, utils::key_to_world_coord_f32, data::voxel_octree::VoxelMode};
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
    .add_startup_system(add_mesh_to_origin)
    .add_startup_system(startup_voxel_scale_1_0)
    .add_startup_system(startup_voxel_scale_0_5)
    .add_system(show_diagnostic_texts)
    .run();
}

fn setup_camera(
  mut commands: Commands,
  local_res: Res<LocalResource>,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_translation(local_res.cam_pos)
        .looking_to(local_res.cam_forward, Vec3::Y),
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
    transform: Transform::from_xyz(6.0, 15.0, 6.0),
    ..Default::default()
  });
}

fn add_mesh_to_origin(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(PbrBundle {
      mesh: meshes.add(shape::Plane::from_size(1.0).into()),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      transform: Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)),
    ..default()
  });
}

fn startup_voxel_scale_1_0(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut local_res: ResMut<LocalResource>,
) {
  let scale = 1.0;
  let config = local_res.chunk_manager.config.clone();
  let mut voxel_reuse = local_res.chunk_manager.voxel_reuse.clone();

  let keys = adj_keys_by_scale([0, 0, 0], 1, scale);
  for key in keys.iter() {
    let mut chunk = Chunk::default();
    if key[1] == 0 {
      let max = chunk.octree.size;
      for x in 0..max {
        for y in 0..max {
          for z in 0..max {

            if y < 3 {
              chunk.octree.set_voxel(x, y, z, 1);
            }
            
          }
        }
      }
    }

    local_res.chunk_manager.set_chunk(key, &chunk);

    let data = chunk
      .octree
      .compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut voxel_reuse,
        &local_res.colors,
        scale
      );

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));
    let mesh_handle = meshes.add(render_mesh);

    let mut pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
    pos_f32[0] *= scale;
    pos_f32[1] = 1.0;
    pos_f32[2] *= scale;
    
    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(pos_f32[0], pos_f32[1], pos_f32[2]),
        ..default()
      })
      .insert(ChunkGraphics {});
  }
  
}

fn startup_voxel_scale_0_5(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut local_res: ResMut<LocalResource>,
) {
  let scale = 0.5;
  let config = local_res.chunk_manager.config.clone();
  let mut voxel_reuse = local_res.chunk_manager.voxel_reuse.clone();

  let keys = adj_keys_by_scale([0, 0, 0], 1, scale);
  for key in keys.iter() {
    let mut chunk = Chunk::default();
    if key[1] == 0 {
      let max = chunk.octree.size;
      for x in 0..max {
        for y in 0..max {
          for z in 0..max {

            if y < 3 {
              chunk.octree.set_voxel(x, y, z, 1);
            }
            
          }
        }
      }
    }
    local_res.chunk_manager.set_chunk(key, &chunk);

    let data = chunk
      .octree
      .compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut voxel_reuse,
        &local_res.colors,
        scale
      );

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    let mesh_handle = meshes.add(render_mesh);

    let mut pos_f32 = key_to_world_coord_f32(key, config.seamless_size);
    pos_f32[0] *= scale;
    pos_f32[1] = 1.0;
    pos_f32[2] *= scale;
    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        transform: Transform::from_xyz(pos_f32[0], pos_f32[1], pos_f32[2]),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
  chunk_manager: ChunkManager,
  colors: Vec<[f32; 3]>,
  cam_pos: Vec3,
  cam_forward: Vec3,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
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
      cam_pos: Vec3::new(-36.82, 27.69, -12.72),
      cam_forward: Vec3::new(0.74, -0.53, 0.39),
    }
  }
}

#[derive(Component, Clone)]
struct ChunkGraphics { }


/*
  Compare the size of meshes based on scale:
    1.0
    0.5
    0.25

  Setup visual checker
*/



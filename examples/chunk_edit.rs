use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PresentMode, PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use utils::RayUtils;
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
    .add_startup_system(setup_starting_chunks)
    .add_system(detect_voxel_preview_position)
    .add_system(on_preview_changed)
    .add_system(show_diagnostic_texts)
    .run();

}

fn setup_camera(
  mut commands: Commands,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(6.41, 42.63, -26.86)
        .looking_to(Vec3::new(0.01, -0.81, 0.57), Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam)
    .insert(Preview::default());

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
}

fn setup_starting_chunks(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut local_res: ResMut<LocalResource>,
) {
  
  let colors = local_res.colors.clone();

  let chunk_manager = &mut local_res.chunk_manager;
  let depth = chunk_manager.depth;
  let voxel_scale = chunk_manager.voxel_scale;

  let seamless_size = chunk_manager.seamless_size();
  let mut voxel_reuse = VoxelReuse::new(depth as u32, 3);
  

  let adj_keys = adjacent_keys(&[0, 0, 0], 1, true);

  let pos_adj = [0.0, 0.0, 0.0];
  for key in adj_keys.iter() {
    let chunk = chunk_manager.new_chunk3(key, depth as u8);
    chunk_manager.set_chunk(key, &chunk);

    let data = chunk
      .octree
      .compute_mesh(
        VoxelMode::SurfaceNets, 
        &mut voxel_reuse,
        &colors,
        voxel_scale
      );

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    let mesh_handle = meshes.add(render_mesh);

    let mut coord_f32 = key_to_world_coord_f32(key, seamless_size);
    coord_f32[0] *= voxel_scale;
    coord_f32[1] *= voxel_scale;
    coord_f32[2] *= voxel_scale;
    
    coord_f32[0] += pos_adj[0];
    coord_f32[1] += pos_adj[1];
    coord_f32[2] += pos_adj[2];

    let mut color = Color::rgb(0.7, 0.7, 0.7);
    if key[0] == 0 && key[2] == 0 {
      color = Color::rgb(1.0, 0.0, 0.0);
    }

    commands
      .spawn(MaterialMeshBundle {
        mesh: mesh_handle,
        material: materials.add(color.into()),
        transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
      })
      .insert(ChunkGraphics {});
  }
}


fn detect_voxel_preview_position(
  mut commands: Commands,
  mut local_res: ResMut<LocalResource>,
  mut cam: Query<(&Transform, &mut Preview), With<FlyCam>>,
) {
  let voxel_scale = local_res.chunk_manager.voxel_scale;

  let max_dist = 100.0;
  let total_div = max_dist as i64 * 2;
  let min_dist = 1.0;
  
  let mut pos = None;
  for (cam_trans, mut preview) in &mut cam {
    'main: for i in 0..total_div {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (max_dist / div_f32) * i as f32;
      if dist < min_dist {
        continue;
      }

      let t = cam_trans.translation;
      let f = cam_trans.forward();
      let p = RayUtils::get_normal_point_with_scale(
        [t.x, t.y, t.z], [f.x, f.y, f.z], dist, voxel_scale
      );

      let p_i64 = [p[0] as i64, p[1] as i64, p[2] as i64];
      let res = local_res.chunk_manager.get_voxel_safe(&p_i64);

      if res.is_some() && res.unwrap() != 0 {
        pos = Some(p);
        // println!("p {:?}", pos);
        break 'main;
      }
    }

    if pos.is_none() && preview.voxel_pos.is_some() {
      preview.voxel_pos = pos;
    }

    if pos.is_some() {
      
      if preview.voxel_pos.is_some() {
        let p = pos.unwrap();
        let current = preview.voxel_pos.unwrap();
        if current != p {
          preview.voxel_pos = pos;
        }
      }
      
      if preview.voxel_pos.is_none() {
        preview.voxel_pos = pos;
      }
    }
  }
}

fn on_preview_changed(
  previews: Query<&mut Preview, Changed<Preview>>,
) {
  for preview in &previews {
    println!("voxel_pos {:?}", preview.voxel_pos);
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
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::new(4, 1.0, 1),
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
struct Preview {
  voxel_pos: Option<[f32; 3]>,
}

impl Default for Preview {
  fn default() -> Self {
    Self {
      voxel_pos: None,
    }
  }
}


#[derive(Component, Clone)]
struct ChunkGraphics { }


/*
  Refactor ChunkManager

  Implement chunk range
  Implement voxel per chunk

*/

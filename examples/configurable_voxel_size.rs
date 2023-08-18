use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PresentMode, PrimaryWindow, CursorGrabMode}, input::mouse::MouseWheel};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys}, utils::key_to_world_coord_f32, data::voxel_octree::VoxelMode};
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
    .add_startup_system(startup)
    .add_startup_system(startup_voxel_preview)
    .add_system(voxel_preview)
    .add_system(voxel_edit)
    .add_system(show_diagnostic_texts)
    .add_system(set_ray_dist)
    .run();

}

fn setup_camera(
  mut commands: Commands,
  mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.9, 2.23, -5.7)
        .looking_to(Vec3::new(-0.075, -0.145, 0.986), Vec3::Y),
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

  let mut window = match windows.get_single_mut() {
    Ok(w) => { w },
    Err(_e) => return,
  };

  // window.cursor.grab_mode = CursorGrabMode::None;
}

fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut local_res: ResMut<LocalResource>,
) {
  let scale = local_res.scale;

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

  let config = local_res.chunk_manager.config.clone();
  let mut voxel_reuse = local_res.chunk_manager.voxel_reuse.clone();
  let mut chunk = Chunk::default();

  let size = (1.0 / scale) as u32;
  println!("size {}", size);
  let start = 2;
  for x in 0..size {
    for y in 0..size {
      for z in 0..size  {
        chunk.octree.set_voxel(start + x, start + y, start + z, 1);
      }
    }
  }

  
  let key = &[0, 0, 0];
  local_res.chunk_manager.set_chunk(key, &chunk);

  let data = chunk
    .octree
    .compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut voxel_reuse,
      &colors,
      scale
    );

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  let mesh_handle = meshes.add(render_mesh);

  let mut coord_f32 = key_to_world_coord_f32(key, config.seamless_size);
  coord_f32[0] *= scale;
  coord_f32[1] *= scale;
  coord_f32[2] *= scale;
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
      transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
      // transform: Transform::from_xyz(0.0, 0.0, 0.0),
      ..default()
    })
    .insert(ChunkGraphics {});
}


fn startup_voxel_preview(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  /*
    Make it changeable later
   */
  let size = 1.5;
  let pos = [0.0, 0.0, 0.0];
  let color = Color::rgba(0.0, 1.0, 0.0, 1.0);
  commands
    .spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 1.5, 0.5))),
      material: materials.add(color.into()),
      transform: Transform::from_xyz(pos[0], pos[1], pos[2]),
      ..default()
    });


  commands.spawn(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.1 })),
    material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.5).into()),
    transform: Transform::from_xyz(2.0, 0.0, 0.0),
    ..default()
  })
  .insert(EditPreview {});
}


fn voxel_preview(
  mut local_res: ResMut<LocalResource>,
  mut preview: Query<&mut Transform, With<EditPreview>>,
  cam: Query<&Transform, (With<FlyCam>, Without<EditPreview>)>,
) {
  let max_dist = local_res.ray_dist;
  let total_div = max_dist as i64 * 2;
  let min_dist = 1.0;
  
  let mut pos_op = None;
  for cam_trans in &cam {
    'main: for i in 0..total_div {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (max_dist / div_f32) * i as f32;
      if dist < min_dist {
        continue;
      }

      let t = cam_trans.translation;
      let f = cam_trans.forward();
      let p = utils::RayUtils::get_normal_point_with_scale(
        [t.x, t.y, t.z], [f.x, f.y, f.z], dist, local_res.scale
      );
      let mul = 1.0 / local_res.scale;
      let voxel_x = (p[0] * mul) as i64;
      let voxel_y = (p[1] * mul) as i64;
      let voxel_z = (p[2] * mul) as i64;
      
      let p_i64 = [voxel_x, voxel_y, voxel_z];

      let res = local_res.chunk_manager.get_voxel_safe(&p_i64);
      if res.is_some() && res.unwrap() != 0 {
        let s = local_res.scale;
        pos_op = Some(Vec3::new(p[0], p[1], p[2]));

        local_res.voxel_pos = Some(p_i64);
        break 'main;
      }
    }
  }

  if pos_op.is_some() {
    for mut trans in &mut preview {
      trans.translation = pos_op.unwrap();

      // println!("pos_op {:?}", pos_op.unwrap());
    }
    
  }
}

fn voxel_edit(
  mut local_res: ResMut<LocalResource>,
  mouse: Res<Input<MouseButton>>,

  mut commands: Commands,
  graphics: Query<Entity, With<ChunkGraphics>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mut voxel = None;

  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(0);
  }

  if local_res.voxel_pos.is_none() || voxel.is_none() {
    return;
  }
  let pos = local_res.voxel_pos.unwrap();

  local_res.chunk_manager.set_voxel2(&pos, voxel.unwrap());

  let scale = local_res.scale;
  let colors = local_res.colors.clone();
  let config = local_res.chunk_manager.config.clone();
  let mut voxel_reuse = local_res.chunk_manager.voxel_reuse.clone();

  for entity in &graphics {
    commands.entity(entity).despawn_recursive();
  }


  let key = &[0, 0, 0];
  let chunk = local_res.chunk_manager.get_chunk(key).unwrap();
  let data = chunk
    .octree
    .compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut voxel_reuse,
      &colors,
      scale
    );

  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  let mesh_handle = meshes.add(render_mesh);

  let mut coord_f32 = key_to_world_coord_f32(key, config.seamless_size);
  coord_f32[0] *= scale;
  coord_f32[1] *= scale;
  coord_f32[2] *= scale;
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
      transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
      ..default()
    })
    .insert(ChunkGraphics {});

}

fn set_ray_dist(
  mut mouse_wheel: EventReader<MouseWheel>,
  mut local_res: ResMut<LocalResource>,
  time: Res<Time>,
) {
  for event in mouse_wheel.iter() {
    local_res.ray_dist += event.y.clamp(-1.0, 1.0) * time.delta_seconds() * 50.0;
    local_res.ray_dist = local_res.ray_dist.clamp(1.0, 50.0);
    // println!("ray_dist {}", local_res.ray_dist);
  }
}



fn show_diagnostic_texts(
  cameras: Query<&Transform, With<FlyCam>>,
  mut windows: Query<&mut Window, With<PrimaryWindow>>,
  key_input: Res<Input<KeyCode>>,

  local_res: Res<LocalResource>,

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
  voxel_pos: Option<[i64; 3]>,

  scale: f32,
  colors: Vec<[f32; 3]>,
  ray_dist: f32,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      voxel_pos: None,
      scale: 0.5,
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
      ray_dist: 50.0
    }
  }
}


#[derive(Component, Clone)]
struct EditPreview { }

#[derive(Component, Clone)]
struct ChunkGraphics { }


/*
  Set how to test
  Create a 1x1x1 cube using world size
  Preview cube corresponding to the voxel size

*/



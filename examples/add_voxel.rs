use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PresentMode, PrimaryWindow, CursorGrabMode}, pbr::NotShadowCaster};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use rapier3d::prelude::ColliderHandle;
use utils::RayUtils;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys}, utils::key_to_world_coord_f32, data::{voxel_octree::VoxelMode, surface_nets::VoxelReuse}};
use bevy_flycam::NoCameraAndGrabPlugin;
use bevy_voxel::{BevyVoxelPlugin, BevyVoxelResource, Preview};


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
    .add_plugin(EguiPlugin)
    .add_plugin(NoCameraAndGrabPlugin)
    .add_plugin(BevyVoxelPlugin)
    .insert_resource(BevyVoxelResource::new(
      4, 
      1.0, 
      1, 
      vec![
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
      vec![0, 1, 4, 8, 12],
    ))
    .insert_resource(LocalResource::default())
    .add_startup_system(setup_camera)
    .add_startup_system(setup_starting_chunks)
    .add_system(detect_selected_voxel_position)
    .add_system(detect_preview_voxel_position)
    .add_system(reposition_selected_voxel)
    .add_system(reposition_preview_voxel)
    .add_system(add_voxel)
    .add_system(change_voxel_size)
    .add_system(show_diagnostic_texts)
    .run();

}

fn setup_camera(
  mut commands: Commands,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(0.91, 11.64, -8.82)
        .looking_to(Vec3::new(0.03, -0.80, 0.59), Vec3::Y),
      ..Default::default()
    })
    .insert(FlyCam)
    .insert(Selected::default())
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
    transform: Transform::from_xyz(6.0, 15.0, 6.0),
    ..Default::default()
  });
}

fn setup_starting_chunks(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,

  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  let scale = bevy_voxel_res.chunk_manager.voxel_scale;
  let size = scale + (scale * 0.1);
  // println!("size {}", size);
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(size).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });

  let key = [0, 0, 0];
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
    if data.positions.len() == 0 {
      continue;
    }
    let pos = bevy_voxel_res.get_pos(chunk.key);

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

    let mut color = Color::rgb(0.7, 0.7, 0.7);
    if chunk.key[0] == key[0] && chunk.key[2] == key[0] {
      color = Color::rgb(1.0, 0.0, 0.0);
    }
    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render_mesh),
        material: materials.add(color.into()),
        transform: Transform::from_translation(pos),
        ..default()
      })
      .insert(ChunkGraphics {
        handle: bevy_voxel_res.add_collider(pos, &data)
      }); 
  }

}


fn detect_selected_voxel_position(
  mut cam: Query<(&Transform, &mut Selected), With<FlyCam>>,

  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut selected) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }

    let pos = bevy_voxel_res.get_hit_voxel_pos(hit.unwrap());
    // println!("pos {:?}", pos);

    if pos.is_none() && selected.pos.is_some() {
      selected.pos = pos;
    }

    if pos.is_some() {
      if selected.pos.is_some() {
        let p = pos.unwrap();
        let current = selected.pos.unwrap();
        if current != p {
          selected.pos = pos;
        }
      }
      
      if selected.pos.is_none() {
        selected.pos = pos;
      }
    }
  }
}

fn detect_preview_voxel_position(
  mut cam: Query<(&Transform, &mut Preview), With<FlyCam>>,

  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }
    let point = hit.unwrap();

    let pos = bevy_voxel_res.get_nearest_voxel_air(point);
    // println!("preview {:?}", pos);

    if pos.is_none() && preview.pos.is_some() {
      preview.pos = pos;
    }

    if pos.is_some() {
      if preview.pos.is_some() {
        let p = pos.unwrap();
        let current = preview.pos.unwrap();
        if current != p {
          preview.pos = pos;
        }
      }
      
      if preview.pos.is_none() {
        preview.pos = pos;
      }
    }
  }
}


fn reposition_selected_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  selecteds: Query<&Selected, Changed<Selected>>,
  preview_graphics: Query<Entity, With<SelectedGraphics>>,
) {
  for selected in &selecteds {
    for entity in &preview_graphics {
      commands.entity(entity).despawn_recursive();
    }

    if selected.pos.is_none() {
      continue;
    }
    let p = selected.pos.unwrap();
    
    let scale = bevy_voxel_res.chunk_manager.voxel_scale;
    let size = scale + (scale * 0.1);
    commands.spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: size})),
      material: materials.add(Color::rgba(0.0, 0.0, 1.0, 0.5).into()),
      transform: Transform::from_translation(p),
      ..default()
    })
    .insert(SelectedGraphics { })
    .insert(NotShadowCaster);

  }
}

fn reposition_preview_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  previews: Query<&Preview, Changed<Preview>>,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,
) {
  for preview in &previews {
    for entity in &preview_graphics {
      commands.entity(entity).despawn_recursive();
    }

    if preview.pos.is_none() {
      continue;
    }
    let p = preview.pos.unwrap();
    // println!("preview {:?}", p);
    // let chunk = bevy_voxel_res.get_preview_chunk(p);
    let chunk = bevy_voxel_res.get_preview(p, preview);
    let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, &chunk);
    
    let pos = bevy_voxel_res.get_preview_pos(p);

    let mut render = Mesh::new(PrimitiveTopology::TriangleList);
    render.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
    render.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
    render.set_indices(Some(Indices::U32(data.indices.clone())));

    commands
      .spawn(MaterialMeshBundle {
        mesh: meshes.add(render),
        material: materials.add(Color::rgba(0.7, 0.7, 0.7, 0.8).into()),
        transform: Transform::from_translation(pos),
        ..default()
      })
      .insert(PreviewGraphics { })
      .insert(NotShadowCaster);
  }
}


fn add_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mouse: Res<Input<MouseButton>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,

  previews: Query<(Entity, &Preview)>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,
) {
  let mut voxel = None;
  if mouse.just_pressed(MouseButton::Left) {
    voxel = Some(1);
  }
  if voxel.is_none() {
    return;
  }

  for (_, preview) in &previews {
    if preview.pos.is_none() {
      continue;
    }
    for (entity, graphics) in &chunk_graphics {
      commands.entity(entity).despawn_recursive();
      bevy_voxel_res.physics.remove_collider(graphics.handle);
    }

    let p = preview.pos.unwrap();
    let pos = bevy_voxel_res.get_nearest_voxel_air(p).unwrap();

    bevy_voxel_res.set_voxel(pos, voxel.unwrap());
    
    let key = [0, 0, 0];
    let chunks = bevy_voxel_res.load_adj_chunks(key);
    for chunk in chunks.iter() {
      let data = bevy_voxel_res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      let pos = bevy_voxel_res.get_pos(chunk.key);

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      let mut color = Color::rgb(0.7, 0.7, 0.7);
      if chunk.key[0] == key[0] && chunk.key[2] == key[0] {
        color = Color::rgb(1.0, 0.0, 0.0);
      }
      commands
        .spawn(MaterialMeshBundle {
          mesh: meshes.add(render_mesh),
          material: materials.add(color.into()),
          transform: Transform::from_translation(pos),
          ..default()
        })
        .insert(ChunkGraphics {
          handle: bevy_voxel_res.add_collider(pos, &data)
        }); 
    }
  }
}

fn change_voxel_size(
  key_input: Res<Input<KeyCode>>,
  mut previews: Query<&mut Preview>,
) {
  if key_input.just_pressed(KeyCode::Equals) {
    for mut params in previews.iter_mut() {
      if params.level < 3 {
        params.level += 1;
        params.size = 2_u8.pow(params.level as u32);
      }
    }
  }

  if key_input.just_pressed(KeyCode::Minus) {
    for mut params in previews.iter_mut() {
      if params.level > 0 {
        params.level -= 1;
        params.size = 2_u8.pow(params.level as u32);
      }
    }
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
}

impl Default for LocalResource {
  fn default() -> Self {
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

    Self {
      chunk_manager: ChunkManager::new(4, 1.0, 1, colors),
    }
  }
}


#[derive(Component, Clone)]
struct Selected {
  pos: Option<Vec3>,
}

impl Default for Selected {
  fn default() -> Self {
    Self {
      pos: None,
    }
  }
}

#[derive(Component, Clone)]
struct SelectedGraphics { }

#[derive(Component, Clone)]
struct PreviewGraphics { }


#[derive(Component, Clone)]
struct ChunkGraphics {
  pub handle: ColliderHandle,
}
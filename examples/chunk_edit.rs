use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}, window::{PresentMode, PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiPlugin, EguiContexts, egui::{Color32, Frame, Rect, Pos2, RichText, Style, Vec2}};
use bevy_flycam::FlyCam;
use utils::RayUtils;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys}, utils::key_to_world_coord_f32, data::{voxel_octree::VoxelMode, surface_nets::VoxelReuse}};
use bevy_flycam::NoCameraAndGrabPlugin;
use bevy_voxel::{BevyVoxelPlugin, BevyVoxelResource};


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
    .add_plugin(BevyVoxelPlugin)
    .add_startup_system(setup_camera)
    .add_startup_system(setup_starting_chunks)
    .add_system(detect_voxel_preview_position)
    .add_system(reposition_voxel_preview)
    .add_system(edit_voxel)
    .add_system(show_diagnostic_texts)
    .run();

}

fn setup_camera(
  mut commands: Commands,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(14.30, 9.50, -25.82)
        .looking_to(Vec3::new(0.09, -0.46, 0.88), Vec3::Y),
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
    transform: Transform::from_xyz(6.0, 15.0, 6.0),
    ..Default::default()
  });
}

fn setup_starting_chunks(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut local_res: ResMut<LocalResource>,

  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {

  let key = [0, 0, 0];
  let chunks = bevy_voxel_res.load_adj_chunks(key);
  for chunk in chunks.iter() {
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
        transform: Transform::from_translation(pos),
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

fn reposition_voxel_preview(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  local_res: Res<LocalResource>,

  previews: Query<&Preview, Changed<Preview>>,
  preview_graphics: Query<Entity, With<PreviewGraphics>>,
) {

  // let voxel_scale = local_res.chunk_manager.voxel_scale;

  // for preview in &previews {
  //   println!("voxel_pos {:?}", preview.voxel_pos);

  //   let mut tmp_chunk_manager = local_res.chunk_manager.clone();

  //   for entity in &preview_graphics {
  //     commands.entity(entity).despawn_recursive();
  //   }

  //   if preview.voxel_pos.is_none() {
  //     continue;
  //   }
  //   let pos = preview.voxel_pos.unwrap();
  //   let voxel_pos = [
  //     pos[0] as i64,
  //     pos[1] as i64,
  //     pos[2] as i64,
  //   ];
  //   tmp_chunk_manager.set_voxel2(&voxel_pos, 1);

  //   let mut chunk = Chunk::default();
  //   let mid_pos = (chunk.octree.get_size() / 2) as i64;

  //   let preview_size = 3;
  //   let min = -preview_size;
  //   let max = preview_size;
  //   for x in min..max {
  //     for y in min..max {
  //       for z in min..max {
  //         let local_x = (mid_pos + x) as u32;
  //         let local_y = (mid_pos + y) as u32;
  //         let local_z = (mid_pos + z) as u32;

  //         let voxel_pos = [
  //           pos[0] as i64 + x,
  //           pos[1] as i64 + y,
  //           pos[2] as i64 + z,
  //         ];
  //         let voxel = tmp_chunk_manager.get_voxel(&voxel_pos);
          
  //         chunk.octree.set_voxel(local_x, local_y, local_z, voxel);
  //       }
  //     }
  //   }

  //   let data = chunk.octree.compute_mesh(
  //     VoxelMode::SurfaceNets, 
  //     &mut VoxelReuse::default(), 
  //     &local_res.colors,
  //     voxel_scale
  //   );

  //   let mut render = Mesh::new(PrimitiveTopology::TriangleList);
  //   render.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  //   render.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  //   render.set_indices(Some(Indices::U32(data.indices.clone())));
    
  //   let v_pos = preview.voxel_pos.unwrap();
  //   let mut pos = [
  //     v_pos[0] + -(mid_pos) as f32,
  //     v_pos[1] + -(mid_pos) as f32,
  //     v_pos[2] + -(mid_pos) as f32,
  //   ];

  //   commands
  //     .spawn(MaterialMeshBundle {
  //       mesh: meshes.add(render),
  //       material: materials.add(Color::rgba(1.0, 1.0, 1.0, 1.0).into()),
  //       transform: Transform::from_xyz(pos[0], pos[1], pos[2]),
  //       ..default()
  //     })
  //     .insert(PreviewGraphics { });

    // commands.spawn(PbrBundle {
    //   mesh: meshes.add(Mesh::from(shape::Cube { size: 1.1 })),
    //   material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    //   transform: Transform::from_xyz(v_pos[0], v_pos[1], v_pos[2]),
    //   ..default()
    // })
    // .insert(PreviewGraphics { });
  // }

  /*
    Defer validation
    Place a voxel here
   */

}


fn edit_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mouse: Res<Input<MouseButton>>,

  mut local_res: ResMut<LocalResource>,
  previews: Query<(Entity, &Preview)>,
  chunk_graphics: Query<Entity, With<ChunkGraphics>>,
) {
  // let mut voxel = None;
  // if mouse.just_pressed(MouseButton::Left) {
  //   voxel = Some(1);

  //   println!("voxel {:?}", voxel);
  // }
  // if voxel.is_none() {
  //   return;
  // }

  // for entity in &chunk_graphics {
  //   commands.entity(entity).despawn_recursive();
  // }


  // for (_, preview) in &previews {
  //   println!("Edit1");
  //   if preview.voxel_pos.is_none() {
  //     continue;
  //   }
  //   let p = preview.voxel_pos.unwrap();
  //   let pos = [p[0] as i64, p[1] as i64, p[2] as i64];
  //   local_res.chunk_manager.set_voxel2(&pos, voxel.unwrap());


  //   println!("Edit2");

  //   let colors = local_res.colors.clone();

  //   let chunk_manager = &mut local_res.chunk_manager;
  //   let depth = chunk_manager.depth;
  //   let voxel_scale = chunk_manager.voxel_scale;

  //   let seamless_size = chunk_manager.seamless_size();
  //   let mut voxel_reuse = VoxelReuse::new(depth as u32, 3);
    

  //   let adj_keys = adjacent_keys(&[0, 0, 0], 1, true);

  //   for key in adj_keys.iter() {
  //     let chunk = chunk_manager.get_chunk(key).unwrap();

  //     let data = chunk
  //       .octree
  //       .compute_mesh(
  //         VoxelMode::SurfaceNets, 
  //         &mut voxel_reuse,
  //         &colors,
  //         voxel_scale
  //       );

  //     let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  //     render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
  //     render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
  //     render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

  //     let mesh_handle = meshes.add(render_mesh);

  //     let mut coord_f32 = key_to_world_coord_f32(key, seamless_size);
  //     coord_f32[0] *= voxel_scale;
  //     coord_f32[1] *= voxel_scale;
  //     coord_f32[2] *= voxel_scale;

  //     let color = Color::rgb(0.7, 0.7, 0.7);
  //     // if key[0] == 0 && key[2] == 0 {
  //     //   color = Color::rgb(1.0, 0.0, 0.0);
  //     // }

  //     commands
  //       .spawn(MaterialMeshBundle {
  //         mesh: mesh_handle,
  //         material: materials.add(color.into()),
  //         transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
  //         ..default()
  //       })
  //       .insert(ChunkGraphics {});
  //   }
  // }

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
struct PreviewGraphics { }


#[derive(Component, Clone)]
struct ChunkGraphics { }


/*
  Refactor ChunkManager

  Implement chunk range
  Implement voxel per chunk

  Position of the preview
    There are some conditions
    Should be either be modified by state


  Types of features
    Dynamic switching: Only one state at a time
      Can be called cartridge
    Parallel: Can co-exists with other features
    Compile time conditional: Minimum graphics vs Normal graphics for faster compilation
    
*/

use bevy::{prelude::*, render::{render_resource::{PrimitiveTopology, VertexFormat}, mesh::{Indices, MeshVertexAttribute}}, window::{PrimaryWindow, CursorGrabMode}};
use bevy_egui::{EguiContexts, egui::{self, TextureId, Frame, Color32, Style, ImageButton, Rect, Vec2, Pos2, RichText}, EguiPlugin};
use bevy_flycam::FlyCam;
use voxels::{data::{voxel_octree::{VoxelOctree, ParentValueType, VoxelMode}, surface_nets::VoxelReuse}, utils::key_to_world_coord_f32, chunk::chunk_manager::ChunkManager};
use noise::*;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(EguiPlugin)
      .add_startup_system(setup_camera)
      .add_startup_system(startup)
      .add_startup_system(test_fast_surface_net)
      .add_system(update);
  }
}

fn setup_camera(
  mut commands: Commands,
) {
  commands
    .spawn(Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 4.0, 4.0)
        .looking_to(Vec3::new(0.7, -0.4, 0.6), Vec3::Y),
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

  let colors = vec![
    [0.0, 0.0, 0.0], 
    [1.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0], 
    [0.0, 0.0, 1.0],

    [0.2, 0.0, 0.0],
    [0.4, 0.0, 0.0],
    [0.6, 0.0, 0.0],
    [0.8, 0.0, 0.0],

    [0.0, 0.2, 0.0],
    [0.0, 0.4, 0.0],
  ];

  let data = chunk
    .octree
    .compute_mesh(
      VoxelMode::SurfaceNets, 
      &mut manager.voxel_reuse,
      &colors,
    );

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


fn test_fast_surface_net(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {

  let noise = OpenSimplex::new().set_seed(1234);

  use fast_surface_nets::ndshape::{ConstShape, ConstShape3u32};
  use fast_surface_nets::{surface_nets, SurfaceNetsBuffer};

  // A 16^3 chunk with 1-voxel boundary padding.
  type ChunkShape = ConstShape3u32<18, 18, 18>;

  // This chunk will cover just a single octant of a sphere SDF (radius 15).
  let mut sdf = [1.0; ChunkShape::USIZE];
  for i in 0u32..ChunkShape::SIZE {
    let [x, y, z] = ChunkShape::delinearize(i);
    // info!("test");
    let elevation = elevation(&x, &z, &0, noise);
    let mid = y as i64 - 4;
    if elevation > mid {
      sdf[i as usize] = -1.0;
    }

    if x == 5 && y == 5 && z == 10 {
      sdf[i as usize] = -1.0;
    }

    if x == 2 && y == 3 && z == 10 {
      sdf[i as usize] = 1.0;
    }
  }

  let mut buffer = SurfaceNetsBuffer::default();
  surface_nets(&sdf, &ChunkShape {}, [0; 3], [17; 3], &mut buffer);


  let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, buffer.positions);
  render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, buffer.normals);
  render_mesh.set_indices(Some(Indices::U32(buffer.indices)));

  let mesh_handle = meshes.add(render_mesh);

  // let coord_f32 = key_to_world_coord_f32(&[0, 0, 0], manager.config.seamless_size);
  let coord_f32 = [0.0, 0.0, 0.0];
  commands
    .spawn(MaterialMeshBundle {
      mesh: mesh_handle,
      material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
      transform: Transform::from_xyz(coord_f32[0], coord_f32[1], coord_f32[2]),
      ..default()
    });
}

fn elevation(x: &u32, z: &u32, middle: &i64, noise: OpenSimplex) -> i64 {
  let frequency = 0.0125;
  // let frequency = 0.05;
  let height_scale = 16.0;
  let fx = (*x as i64 - middle) as f64 * frequency;
  let fz = (*z as i64 - middle) as f64 * frequency;
  let noise = noise.get([fx, fz]);
  let elevation = (noise * height_scale) as i64;
  elevation
}


fn update(
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

  egui::Window::new("ChunkTexts")
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





#[cfg(test)]
mod tests {
  use std::fs::{File, self};
  use std::io::Write;
  use bevy::prelude::*;
  use bevy::utils::HashMap;
  use voxels::chunk::chunk_manager::{ChunkManager, Chunk};
  use voxels::data::voxel_octree::VoxelOctree;
  use crate::data::{Terrains, Data, Status};


  #[test]
  fn test_modify_save_load_terrains() {
    // Modify
    let mut chunk_manager = ChunkManager::default();

    let target_point = [0, -5, 0];
    let voxel = 0;
    let scale = 4;
    let min = -(scale as i64);
    let max = (scale as i64) + 1;

    let mut modified_chunks = HashMap::new();
    for x in min..max {
      for y in min..max {
        for z in min..max {

          let pos = [
            target_point[0] as i64 + x,
            target_point[1] as i64 + y,
            target_point[2] as i64 + z
          ];
          let chunks = chunk_manager.set_voxel2(&pos, voxel);
          for (key, chunk) in chunks.iter() {
            modified_chunks.insert(key.clone(), chunk.clone());
          }
        }
      }
    }


    // Save to file
    let mut terrains = Terrains { keys: Vec::new(), voxels: Vec::new() };
    for (key, chunk) in modified_chunks.iter() {
      terrains.keys.push(key.clone());
      terrains.voxels.push(array_bytes::bytes2hex("", &chunk.octree.data));
    }

    let mut pos = Vec3::ZERO;
    let data = Data {
      status: Status {
        position: pos.into(),
      },
      terrains: terrains
    };

    let str = toml::to_string_pretty(&data).unwrap();
    let path = std::env::current_dir()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    let file = format!("{}/assets/temp/save.toml", path);
    let mut data = File::create(file.clone()).expect("creation failed");
    data.write(str.as_bytes()).expect("write failed");


    // Load file
    let loaded_str = fs::read_to_string(file.clone())
      .expect("Should have been able to read the file");

    let data: Data = match toml::from_str(&loaded_str) {
      Ok(d) => d,
      Err(_) => Data::default()
    };

    // println!("done {}", loaded_str);
    // println!("{:?}", data);

    let mut load_chunk_manager = ChunkManager::default();
    for i in 0..data.terrains.keys.len() {
      let key = &data.terrains.keys[i];
      let voxels_str = &data.terrains.voxels[i];
      let voxels_res = array_bytes::hex2bytes(voxels_str);
      if voxels_res.is_ok() {
        let data = voxels_res.unwrap();
        let octree = VoxelOctree::new_from_bytes(data);
        let chunk = Chunk {
          key: key.clone(),
          octree: octree,
          is_default: false,
          ..Default::default()
        };
        load_chunk_manager.set_chunk(key, &chunk);

        // info!("load data key {:?}", key);
      }
    }


    // Compare values from ChunkManager
    let range_check = 20;
    let starting_coord = [0, 0, 0];
    let min = -range_check;
    let max = range_check;
    for x in min..max {
      for y in min..max {
        for z in min..max {
          let coord_x = starting_coord[0] - x;
          let coord_y = starting_coord[1] - y;
          let coord_z = starting_coord[2] - z;

          let pos = [coord_x, coord_y, coord_z];
          let val0 = chunk_manager.get_voxel(&pos);
          let val1 = load_chunk_manager.get_voxel(&pos);

          if val0 != val1 {
            println!("not equal");
          }
          assert_eq!(val0, val1);
        }
      }
    }

    // Compare values of Octrees in ChunkManager
    for (key, chunk) in chunk_manager.chunks.iter() {
      let chunk1 = load_chunk_manager.chunks.get(key).unwrap();
      let size = chunk.octree.size;
      for x in 0..size {
        for y in 0..size {
          for z in 0..size {
            let val0 = chunk.octree.get_voxel(x, y, z);
            let val1 = chunk1.octree.get_voxel(x, y, z);

            // println!("{} {} {}", x, y, z);

            assert_eq!(val0, val1);
          }
        }
      }
    }



  }
}



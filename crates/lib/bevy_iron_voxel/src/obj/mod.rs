use bevy::prelude::*;
// use voxels::chunk::chunk_manager::ChunkManager;
// use obj_exporter::*;
use crate::{data::GameResource, components::player::Player};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, export);
  }
}

fn export(
  _keys: Res<Input<KeyCode>>,
  mut _game_res: ResMut<GameResource>,
  _players: Query<&Player>,
) {
  // if keys.just_pressed(KeyCode::Period) {
  //   let mut player_key = [0, 0, 0];
  //   for player in players.iter() {
  //     player_key = player.key;
  //   }
  //   info!("Export to OBJ at key {:?}", player_key);

  //   let mut pos = Vec::new();
  //   let mut uv = Vec::new();
  //   let mut normals = Vec::new();
  //   let mut indices = Vec::new();

  //   let mut voxel_reuse = game_res.chunk_manager.voxel_reuse.clone();
  //   let config = game_res.chunk_manager.config.clone();

  //   let adj_keys = adjacent_keys(&player_key, 1, true);

  //   let mut chunks = HashMap::new();
  //   for key in adj_keys.iter() {
  //     let c = game_res
  //       .chunk_manager
  //       .get_chunk(key)
  //       .unwrap()
  //       .clone();
  //     chunks.insert(key.clone(), c);
  //   }

  //   let mut last_index = 0;
  //   for (key, chunk) in chunks.iter() {
  //     let mut mesh = chunk.octree.compute_mesh(
  //       VoxelMode::SurfaceNets, 
  //       &mut voxel_reuse,
  //       &game_res.colors,
  //       1.0,
  //     );

  //     if mesh.positions.len() == 0 {
  //       continue;
  //     }

  //     let diff = [
  //       key[0] - player_key[0],
  //       key[1] - player_key[1],
  //       key[2] - player_key[2],
  //     ];

  //     for p in mesh.positions.iter() {
  //       let px = p[0] + (diff[0] * config.seamless_size as i64) as f32;
  //       let py = p[1] + (diff[1] * config.seamless_size as i64) as f32;
  //       let pz = p[2] + (diff[2] * config.seamless_size as i64) as f32;

  //       pos.push([px, py, pz]);
  //     }

  //     uv.append(&mut mesh.uvs);
  //     normals.append(&mut mesh.normals);

  //     let mut ind: Vec<(usize, usize, usize)> = mesh.indices.chunks(3)
  //       .into_iter()
  //       .map(|c| (
  //         c[0] as usize + last_index,
  //         c[1] as usize + last_index, 
  //         c[2] as usize + last_index
  //       ))
  //       .collect();
      
  //     last_index += mesh.indices.len();
  //     indices.append(&mut ind);
  //   }

  //   let set = ObjSet {
  //     material_library: None,
  //     objects: vec![
  //       Object {
  //         name: "Cube".to_owned(),
  //         vertices: pos
  //           .into_iter()
  //           .map(|[x, y, z]| Vertex { x: x as f64, y: y as f64, z: z as f64 })
  //           .collect(),
  //         tex_vertices: uv
  //           .into_iter()
  //           .map(|[u, v]| TVertex { u: u as f64, v: v as f64, w: 0.0 })
  //           .collect(),
  //         normals: normals
  //           .into_iter()
  //           .map(|[x, y, z]| Vertex { x: x as f64, y: y as f64, z: z as f64 })
  //           .collect(),
  //         geometry: vec![
  //           Geometry {
  //             material_name: None,
  //             shapes: indices
  //               .into_iter()
  //               .map(|(x, y, z)|
  //                 Shape {
  //                   primitive:
  //                     Primitive::Triangle(
  //                       (x, None, None),
  //                       (y, None, None),
  //                       (z, None, None)),
  //                   groups: vec!(),
  //                   smoothing_groups: vec!()
  //                 })
  //               .collect(),
  //           },
  //         ],        
  //       },
  //     ],
  //   };

    
  //   let mut output = Vec::<u8>::new();
  //   obj_exporter::export(&set, &mut output).unwrap();
  //   let str = String::from_utf8(output).unwrap();

  //   game_res.export_obj = Some(str.to_string());

  //   info!("Prepare export");
  // }

/* 
    let main_dir = std::env::current_dir().unwrap();
    let main_str = main_dir.to_str().unwrap();
    let path = format!("{}/assets", main_str);

    let res = rfd::FileDialog::new()
      .set_file_name("save.obj")
      .set_directory(&path)
      .save_file();

    if res.is_none() {
      return;
    }

    let p = res.unwrap();
    let mut data_file = File::create(p).expect("creation failed");
    use std::io::Write;
    data_file.write(str.as_bytes()).expect("write failed");
     */


    // For testing
/*     
    let path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
    let file = format!("{}/assets/temp/test.obj", path);
    let mut data = File::create(file).expect("creation failed");

    use std::io::Write;
    data.write(str.as_bytes()).expect("write failed");
 */
  
}



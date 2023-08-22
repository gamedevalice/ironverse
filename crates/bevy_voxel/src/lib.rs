mod physics;
mod remove;
mod add;

use bevy::{prelude::*, pbr::NotShadowCaster, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use physics::Physics;
use rapier3d::{prelude::{Vector, ColliderHandle, Ray, QueryFilter}, na::Point3};
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys, voxel_pos_to_key}, data::{voxel_octree::{VoxelMode, MeshData}, surface_nets::VoxelReuse}, utils::key_to_world_coord_f32};
use utils::RayUtils;

pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_state::<EditState>()
      .insert_resource(BevyVoxelResource::default())
      .add_plugin(remove::CustomPlugin)
      .add_plugin(add::CustomPlugin)
      .add_startup_system(startup)
      .add_system(update)
      .add_system(detect_selected_voxel_position)
      .add_system(detect_preview_voxel_position)
      .add_system(added_chunks)
      .add_system(center_changed);
  }
}

fn startup() {
  println!("startup BevyVoxel");
}

fn update(mut res: ResMut<BevyVoxelResource>) {
  res.physics.step();
}

fn detect_selected_voxel_position(
  mut cam: Query<(&Transform, &mut Selected), With<Selected>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut selected) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }

    let pos = bevy_voxel_res.get_hit_voxel_pos(hit.unwrap());
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
  mut cam: Query<(&Transform, &mut Preview), With<Preview>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  for (cam_trans, mut preview) in &mut cam {
    let hit = bevy_voxel_res.get_raycast_hit(cam_trans);
    if hit.is_none() {
      continue;
    }
    let point = hit.unwrap();
    let pos = bevy_voxel_res.get_nearest_voxel_air(point);
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



fn added_chunks(
  mut res: ResMut<BevyVoxelResource>,
  mut chunks: Query<(&Center, &mut Chunks), Added<Chunks>>
) {
  for (center, mut chunks) in &mut chunks {
    let all_chunks = res.load_adj_chunks_with_collider(center.key);
    for chunk in all_chunks.iter() {
      let data = res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }
}

fn center_changed(
  mut res: ResMut<BevyVoxelResource>,
  mut centers: Query<(&Center, &mut Chunks), Changed<Center>>
) {
  for (center, mut chunks) in &mut centers {
    let all_chunks = res.load_adj_chunks_with_collider(center.key);
    chunks.data.clear();

    for chunk in all_chunks.iter() {
      let data = res.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }
      chunks.data.push(ChunkData {
        data: data.clone(),
        key: chunk.key,
      });
    }
  }
}



#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum EditState {
  AddNormal,
  AddSnap,
  #[default]
  RemoveNormal,
  RemoveSnap,
}



#[derive(Component, Clone)]
pub struct Selected {
  pub pos: Option<Vec3>,
}

impl Default for Selected {
  fn default() -> Self {
    Self {
      pos: None,
    }
  }
}

#[derive(Component, Clone)]
pub struct Preview {
  pub pos: Option<Vec3>,
  pub level: u8,
  pub size: u8,
}

impl Default for Preview {
  fn default() -> Self {
    let level = 1;
    Self {
      pos: None,
      level: level,
      size: 2_u8.pow(level as u32),
    }
  }
}
#[derive(Component, Clone)]
pub struct SelectedGraphics;

#[derive(Component, Clone)]
pub struct PreviewGraphics;



#[derive(Component, Debug, Clone)]
pub struct ChunkData {
  pub key: [i64; 3],
  pub data: MeshData,
}

#[derive(Component, Debug, Clone)]
pub struct Chunks {
  pub data: Vec<ChunkData>,
}

impl Default for Chunks {
  fn default() -> Self {
    Self {
      data: Vec::new(),
    }
  }
}

#[derive(Component)]
pub struct Center {
  pub key: [i64; 3],
}

impl Default for Center {
  fn default() -> Self {
    Self {
      key: [0; 3],
    }
  }
}


#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
  pub physics: Physics,

  colliders_cache: Vec<ColliderHandle>,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
    }
  }
}

impl BevyVoxelResource {

  pub fn new(
    depth: u32, 
    voxel_scale: f32, 
    range: u8,
    colors: Vec<[f32; 3]>,  
  ) -> Self {
    Self {
      chunk_manager: ChunkManager::new(
        depth,
        voxel_scale,
        range,
        colors,
      ),
      physics: Physics::default(),
      colliders_cache: Vec::new(),
    }
  }

  pub fn get_key(&self, pos: Vec3) -> [i64; 3] {
    let scale = self.chunk_manager.voxel_scale;
    let seamless_size = self.chunk_manager.seamless_size();
    get_key(pos, scale, seamless_size)
  }

  /// Get all chunks adjacent to the player based on
  /// Depth, range and voxel scale
  pub fn load_adj_chunks(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    let keys = adjacent_keys(&key, self.chunk_manager.range as i64, true);
    for key in keys.iter() {
      chunks.push(load_chunk(self, *key));
      
    }

    chunks
  }

  /// Return mesh data needed for collision or rendering
  pub fn compute_mesh(&self, mode: VoxelMode, chunk: &Chunk) -> MeshData {
    chunk
      .octree
      .compute_mesh(
        mode, 
        &mut VoxelReuse::new(self.chunk_manager.depth, 3),
        &self.chunk_manager.colors,
        self.chunk_manager.voxel_scale
      )
  }

  /// Return a world position based on chunk size(depth) and voxel scale
  pub fn get_pos(&self, key: [i64; 3]) -> Vec3 {
    let seamless = self.chunk_manager.seamless_size();
    let scale = self.chunk_manager.voxel_scale;
    let mut pos = key_to_world_coord_f32(&key, seamless);

    pos[0] *= scale;
    pos[1] *= scale;
    pos[2] *= scale;
    
    Vec3::new(pos[0], pos[1], pos[2])
  }


  pub fn get_raycast_hit(&self, trans: &Transform) -> Option<Vec3> {
    let start_pos = trans.translation;
    let dir = trans.forward();
    let ray = Ray::new(
      Point3::new(start_pos.x, start_pos.y, start_pos.z), 
      Vector::new(dir.x, dir.y, dir.z)
    );
    let max_toi = f32::MAX;
    let solid = true;
    let filter = QueryFilter::only_fixed();

    let mut point = None;
    if let Some((_handle, toi)) = self.physics.query_pipeline.cast_ray(
      &self.physics.rigid_body_set, 
      &self.physics.collider_set, 
      &ray, 
      max_toi, 
      solid, 
      filter
    ) {
      let hit = ray.point_at(toi);
      point = Some(Vec3::new(hit[0], hit[1], hit[2]));
    }
    point
  }

  pub fn get_hit_voxel_pos(&self, point: Vec3) -> Option<Vec3> {
    let voxel_scale = self.chunk_manager.voxel_scale;
    let mul = 1.0 / voxel_scale;
    let mut nearest_dist = f32::MAX;

    let mut pos = None;
    let n_c = RayUtils::get_nearest_coord(
      [point.x, point.y, point.z], voxel_scale
    );

    let tmp_point = Vec3::new(n_c[0], n_c[1], n_c[2]);
    let near_pos = get_near_positions(tmp_point, voxel_scale);
    for n in near_pos.iter() {
      let dist = point.distance(*n);
      let tmp_pos = [
        (n[0] * mul) as i64, 
        (n[1] * mul) as i64, 
        (n[2] * mul) as i64, 
      ];

      let res = self.chunk_manager.get_voxel_safe(&tmp_pos);
      if res.is_some() && res.unwrap() != 0 {
        if dist < nearest_dist {
          nearest_dist = dist;
          pos = Some(*n);
        }
      }
    }

    pos
  }

  pub fn get_nearest_voxel_air(&self, point: Vec3) -> Option<Vec3> {
    let voxel_scale = self.chunk_manager.voxel_scale;

    let mul = 1.0 / voxel_scale;
    let mut nearest_dist = f32::MAX;

    let mut pos = None;
    let n_c = RayUtils::get_nearest_coord(
      [point.x, point.y, point.z], voxel_scale
    );

    let tmp_point = Vec3::new(n_c[0], n_c[1], n_c[2]);
    let near_pos = get_near_positions(tmp_point, voxel_scale);
    
    for n in near_pos.iter() {
      let dist = point.distance(*n);
      let tmp_pos = [
        (n[0] * mul) as i64, 
        (n[1] * mul) as i64, 
        (n[2] * mul) as i64, 
      ];

      let res = self.chunk_manager.get_voxel_safe(&tmp_pos);
      if res.is_some() && res.unwrap() == 0 {
        if dist < nearest_dist {
          nearest_dist = dist;
          pos = Some(*n);
        }
      }
    }

    // println!("point {:?} : {:?}", point, pos);
    pos
  }

  /// - calc_pos should be the calculated position based on edit mode
  /// - Add voxel mode(TODO): Probably be a separate function
  /// - Remove voxel mode(TODO): Probably be a separate function
  pub fn get_preview_chunk(&self, calc_pos: Vec3) -> Chunk {
    let mul = (1.0 / self.chunk_manager.voxel_scale);
    let p = [
      calc_pos.x * mul,
      calc_pos.y * mul,
      calc_pos.z * mul,
    ];

    let mut tmp_manager = self.chunk_manager.clone();
    set_voxel(&mut tmp_manager, calc_pos, 1);

    let mut chunk = Chunk::default();
    let mid_pos = (chunk.octree.get_size() / 2) as i64;
    
    let preview_size = 3;
    let min = -preview_size;
    let max = preview_size;
    for x in min..max {
      for y in min..max {
        for z in min..max {
          let local_x = (mid_pos + x) as u32;
          let local_y = (mid_pos + y) as u32;
          let local_z = (mid_pos + z) as u32;

          let tmp_pos = [
            p[0] as i64 + x,
            p[1] as i64 + y,
            p[2] as i64 + z,
          ];
          let voxel = tmp_manager.get_voxel(&tmp_pos);
          chunk.octree.set_voxel(local_x, local_y, local_z, voxel);
        }
      }
    }
    
    chunk
  }

  /// Get preview chunk pos converted to world pos considering the size of chunk
  /// and positioned visually correct
  pub fn get_preview_pos(&self, calc_pos: Vec3) -> Vec3 {
    let scale = self.chunk_manager.voxel_scale;
    let mid_pos = (self.chunk_manager.chunk_size / 2) as f32 * scale;
    Vec3::new(
      calc_pos.x - mid_pos,
      calc_pos.y - mid_pos,
      calc_pos.z - mid_pos,
    )
  }


  pub fn set_voxel(&mut self, pos: Vec3, voxel: u8) {
    let mul = 1.0 / self.chunk_manager.voxel_scale;
    let p = [
      (pos.x * mul) as i64,
      (pos.y * mul) as i64,
      (pos.z * mul) as i64,
    ];

    self.chunk_manager.set_voxel2(&p, voxel);
  }


  pub fn load_adj_chunks_with_collider(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let chunks = self.load_adj_chunks(key);

    for _ in 0..self.colliders_cache.len() {
      let h = self.colliders_cache.pop().unwrap();
      self.remove_collider(h);
    }    

    self.colliders_cache.clear();

    for chunk in chunks.iter() {
      let data = self.compute_mesh(VoxelMode::SurfaceNets, chunk);
      if data.positions.len() == 0 {
        continue;
      }

      let pos = self.get_pos(chunk.key);
      let c = self.add_collider(pos, &data);
      self.colliders_cache.push(c);
    }

    chunks
  }


  pub fn add_collider(
    &mut self, 
    pos: Vec3, 
    data: &MeshData
  ) -> ColliderHandle {
    self.physics.add_collider(
      [pos.x, pos.y, pos.z], &data.positions, &data.indices
    )
  }

  pub fn remove_collider(&mut self, handle: ColliderHandle) {
    self.physics.remove_collider(handle);
  }


}

fn set_voxel(
  chunk_manager: &mut ChunkManager,
  pos: Vec3,
  voxel: u8,
) {
  let mul = 1.0 / chunk_manager.voxel_scale;
  // let mul = 1.0;
  let p = [
    (pos.x * mul) as i64,
    (pos.y * mul) as i64,
    (pos.z * mul) as i64,
  ];

  chunk_manager.set_voxel2(&p, voxel);
}


fn load_chunk(resource: &mut BevyVoxelResource, key: [i64; 3]) -> Chunk {
  let res = resource.chunk_manager.get_chunk(&key);
  if res.is_none() {
    let chunk = resource.chunk_manager.new_chunk3(&key, resource.chunk_manager.depth as u8);
    resource.chunk_manager.set_chunk(&key, &chunk);
    return chunk;
  }

  res.unwrap().clone()
}


fn get_near_positions(pos: Vec3, unit: f32) -> Vec<Vec3> {
  let mut res = Vec::new();
  let min = -1;
  let max = 2;
  for x in min..max {
    for y in min..max {
      for z in min..max {
        res.push(Vec3::new(
          pos[0] + (x as f32 * unit),
          pos[1] + (y as f32 * unit),
          pos[2] + (z as f32 * unit),
        ));
      }
    }
  }

  res
}


fn get_key(pos: Vec3, voxel_scale: f32, seamless_size: u32) -> [i64; 3] {
  let p = [
    pos.x as i64,
    pos.y as i64,
    pos.z as i64,
  ];

  let div = (1.0 / voxel_scale) as u32;
  let s = seamless_size / div;

  let s1 = (seamless_size as f32) / (1.0 / voxel_scale);
  pos_to_key(pos, s1)
}


pub fn pos_to_key(pos: Vec3, seamless_size: f32) -> [i64; 3] {
  let mut x = pos[0];
  let mut y = pos[1];
  let mut z = pos[2];

  // Between -0.epsilon to -seamless_size..., it should be -1
  if x < 0.0 {
    x -= seamless_size;
  }
  if y < 0.0 {
    y -= seamless_size;
  }
  if z < 0.0 {
    z -= seamless_size;
  }

  [
    (x / seamless_size) as i64,
    (y / seamless_size) as i64,
    (z / seamless_size) as i64,
  ]
}


#[cfg(test)]
mod tests {
  use bevy::prelude::Vec3;
  use voxels::chunk::chunk_manager::ChunkManager;
  use crate::{get_key, get_near_positions};

  #[test]
  fn test_near_positions_1_0() -> Result<(), String> {
    let scale = 1.0;
    let pos = Vec3::new(0.0, 0.0, 0.0);
    let res = get_near_positions(pos, scale);
    let expected = vec![
      Vec3::new(-1.0, -1.0, -1.0),
      Vec3::new(-1.0, -1.0, 0.0),
      Vec3::new(-1.0, -1.0, 1.0),
      Vec3::new(-1.0, 0.0, -1.0),
      Vec3::new(-1.0, 0.0, 0.0),
      Vec3::new(-1.0, 0.0, 1.0),
      Vec3::new(-1.0, 1.0, -1.0),
      Vec3::new(-1.0, 1.0, 0.0),
      Vec3::new(-1.0, 1.0, 1.0),
      Vec3::new(0.0, -1.0, -1.0),
      Vec3::new(0.0, -1.0, 0.0),
      Vec3::new(0.0, -1.0, 1.0),
      Vec3::new(0.0, 0.0, -1.0),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 1.0),
      Vec3::new(0.0, 1.0, -1.0),
      Vec3::new(0.0, 1.0, 0.0),
      Vec3::new(0.0, 1.0, 1.0),
      Vec3::new(1.0, -1.0, -1.0),
      Vec3::new(1.0, -1.0, 0.0),
      Vec3::new(1.0, -1.0, 1.0),
      Vec3::new(1.0, 0.0, -1.0),
      Vec3::new(1.0, 0.0, 0.0),
      Vec3::new(1.0, 0.0, 1.0),
      Vec3::new(1.0, 1.0, -1.0),
      Vec3::new(1.0, 1.0, 0.0),
      Vec3::new(1.0, 1.0, 1.0),
    ];
    
    for p in res.iter() {
      assert!(expected.contains(p));
    }
    Ok(())
  }


  #[test]
  fn test_near_positions_0_5() -> Result<(), String> {
    let scale = 0.5;
    let pos = Vec3::new(0.0, 0.0, 0.0);

    let res = get_near_positions(pos, scale);

    let expected = vec![
      Vec3::new(-0.5, -0.5, -0.5),
      Vec3::new(-0.5, -0.5, 0.0),
      Vec3::new(-0.5, -0.5, 0.5),
      Vec3::new(-0.5, 0.0, -0.5),
      Vec3::new(-0.5, 0.0, 0.0),
      Vec3::new(-0.5, 0.0, 0.5),
      Vec3::new(-0.5, 0.5, -0.5),
      Vec3::new(-0.5, 0.5, 0.0),
      Vec3::new(-0.5, 0.5, 0.5),
      Vec3::new(0.0, -0.5, -0.5),
      Vec3::new(0.0, -0.5, 0.0),
      Vec3::new(0.0, -0.5, 0.5),
      Vec3::new(0.0, 0.0, -0.5),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 0.0, 0.5),
      Vec3::new(0.0, 0.5, -0.5),
      Vec3::new(0.0, 0.5, 0.0),
      Vec3::new(0.0, 0.5, 0.5),
      Vec3::new(0.5, -0.5, -0.5),
      Vec3::new(0.5, -0.5, 0.0),
      Vec3::new(0.5, -0.5, 0.5),
      Vec3::new(0.5, 0.0, -0.5),
      Vec3::new(0.5, 0.0, 0.0),
      Vec3::new(0.5, 0.0, 0.5),
      Vec3::new(0.5, 0.5, -0.5),
      Vec3::new(0.5, 0.5, 0.0),
      Vec3::new(0.5, 0.5, 0.5),
    ];
    
    for p in res.iter() {
      assert!(expected.contains(p));
    }
    
    Ok(())
  }


  #[test]
  fn test_get_key_1_0() -> Result<(), String> {
    let depth = 4;
    let voxel_scale = 1.0;
    let range = 1;
    let manager = ChunkManager::new(depth, voxel_scale, range, Vec::new());
    
    let pos = vec![
      Vec3::new(-27.9, -27.9, -27.9),
      Vec3::new(-13.9, -13.9, -13.9),
      Vec3::new(  0.0,   0.0,   0.0),
      Vec3::new( 14.0,  14.0,  14.0),
      Vec3::new( 28.0,  28.0,  28.0),
    ];
    
    let expected = vec![
      [-2,-2,-2],
      [-1,-1,-1],
      [ 0, 0, 0],
      [ 1, 1, 1],
      [ 2, 2, 2],
    ];
    for (i, p) in pos.iter().enumerate() {
      let key = get_key(*p, voxel_scale, manager.seamless_size());
      assert_eq!(key, expected[i]);
    }
    Ok(())
  }

  #[test]
  fn test_get_key_0_5() -> Result<(), String> {
    let depth = 4;
    let voxel_scale = 0.5;
    let range = 1;
    let manager = ChunkManager::new(depth, voxel_scale, range, Vec::new());
    
    let pos = vec![
      Vec3::new(-13.9, -13.9, -13.9),
      Vec3::new(-6.9,   -6.9,  -6.9),
      Vec3::new( 0.0,    0.0,   0.0),
      Vec3::new( 7.0,    7.0,   7.0),
      Vec3::new( 14.0,  14.0,  14.0),
    ];
    
    let expected = vec![
      [-2,-2,-2],
      [-1,-1,-1],
      [ 0, 0, 0],
      [ 1, 1, 1],
      [ 2, 2, 2],
    ];
    for (i, p) in pos.iter().enumerate() {
      let key = get_key(*p, voxel_scale, manager.seamless_size());
      assert_eq!(key, expected[i]);
    }
    Ok(())
  }

  #[test]
  fn test_get_key_0_25() -> Result<(), String> {
    let depth = 4;
    let voxel_scale = 0.25;
    let range = 1;
    let manager = ChunkManager::new(depth, voxel_scale, range, Vec::new());
    
    let pos = vec![
      Vec3::new(-6.9, -6.9, -6.9),
      Vec3::new(-3.4, -3.4, -3.4),
      Vec3::new( 0.0,  0.0,  0.0),
      Vec3::new( 3.5,  3.5,  3.5),
      Vec3::new( 7.0,  7.0,  7.0),
    ];
    
    let expected = vec![
      [-2,-2,-2],
      [-1,-1,-1],
      [ 0, 0, 0],
      [ 1, 1, 1],
      [ 2, 2, 2],
    ];
    for (i, p) in pos.iter().enumerate() {
      let key = get_key(*p, voxel_scale, manager.seamless_size());
      assert_eq!(key, expected[i]);
    }
    Ok(())
  }


}


/*
  Add voxel
    Nearest voxel, using one voxel
  Remove voxel
    By distance
  
  Component based
    Create basic functions to do the tasks
    Improve, deprecate or create a new functions to accomodate features
*/
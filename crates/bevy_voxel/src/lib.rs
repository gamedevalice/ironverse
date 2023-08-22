mod physics;

use bevy::{prelude::*, pbr::NotShadowCaster, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use physics::Physics;
use rapier3d::{prelude::{Vector, ColliderHandle, Ray, QueryFilter}, na::Point3};
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys, voxel_pos_to_key}, data::{voxel_octree::{VoxelMode, MeshData}, surface_nets::VoxelReuse}, utils::key_to_world_coord_f32};
use utils::RayUtils;

pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(BevyVoxelResource::default())
      .add_startup_system(startup)
      .add_system(update)
      .add_system(detect_selected_voxel_position)
      // .add_system(detect_preview_voxel_position)
      .add_system(reposition_selected_voxel)
      // .add_system(reposition_preview_voxel)
      ;
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

    // println!("hit {:?}", hit.unwrap());

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


fn reposition_selected_voxel(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  bevy_voxel_res: Res<BevyVoxelResource>,

  selecteds: Query<&Selected, Changed<Selected>>,
  selected_graphics: Query<Entity, With<SelectedGraphics>>,
) {
  for selected in &selecteds {
    if selected_graphics.iter().len() == 0 {
      continue;
    }

    for entity in &selected_graphics {
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
    .insert(SelectedGraphics)
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
    if preview_graphics.iter().len() == 0 {
      continue;
    }

    for entity in &preview_graphics {
      commands.entity(entity).despawn_recursive();
    }

    if preview.pos.is_none() {
      continue;
    }
    let p = preview.pos.unwrap();
    // println!("preview {:?}", p);
    let chunk = bevy_voxel_res.get_preview_chunk(p);
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
      .insert(PreviewGraphics)
      .insert(NotShadowCaster);
  }
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




#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
  pub physics: Physics,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
      physics: Physics::default(),
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
    }
  }

  pub fn get_key(&self, pos: Vec3) -> [i64; 3] {
    /*
      TODO: Voxel scale other than 1.0 is not working properly
     */
    let scale = self.chunk_manager.voxel_scale;

    let div = (1.0 / scale) as u32;

    let seamless_size = self.chunk_manager.seamless_size() / div;
    let mul = (1.0 / 1.0) as i64;
    let p = [
      pos.x as i64 * mul,
      pos.y as i64 * mul,
      pos.z as i64 * mul,
    ];

    voxel_pos_to_key(&p, seamless_size)




    /*
      Scale = 0.5
      Seamless size = 12
        0.0  -> 0
        0.49 -> 0
        0.5  -> 1.0
     */
  }

  /// Get all chunks adjacent to the player based on
  /// Depth, range and voxel scale
  pub fn load_adj_chunks(&mut self, key: [i64; 3]) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    let scale = self.chunk_manager.voxel_scale;
    let mul = (1.0 / scale) as i64;

    let adj_key = [
      key[0] * mul,
      key[1] * mul,
      key[2] * mul,
    ];

    let keys = adjacent_keys(&adj_key, self.chunk_manager.range as i64, true);
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
    let voxel_pos = [calc_pos.x as i64, calc_pos.y as i64, calc_pos.z as i64];

    let mut tmp_manager = self.chunk_manager.clone();
    tmp_manager.set_voxel2(&voxel_pos, 1);

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
            voxel_pos[0] as i64 + x,
            voxel_pos[1] as i64 + y,
            voxel_pos[2] as i64 + z,
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

#[cfg(test)]
mod tests {
  use bevy::prelude::Vec3;
  use crate::get_near_positions;

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
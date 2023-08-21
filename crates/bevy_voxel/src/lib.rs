use bevy::prelude::*;
use voxels::{chunk::{chunk_manager::{ChunkManager, Chunk}, adjacent_keys}, data::{voxel_octree::{VoxelMode, MeshData}, surface_nets::VoxelReuse}, utils::key_to_world_coord_f32};
use utils::RayUtils;


pub struct BevyVoxelPlugin;
impl Plugin for BevyVoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(BevyVoxelResource::default())
      .add_startup_system(startup);
  }
}

fn startup() {
  println!("startup BevyVoxel");
}



#[derive(Resource)]
pub struct BevyVoxelResource {
  pub chunk_manager: ChunkManager,
}

impl Default for BevyVoxelResource {
  fn default() -> Self {
    Self {
      chunk_manager: ChunkManager::default(),
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
      )
    }
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


  pub fn get_hit_voxel_pos(&self, trans: &Transform) -> Option<Vec3> {
    let voxel_scale = self.chunk_manager.voxel_scale;
    let t = trans.translation;
    let f = trans.forward();

    let max_dist = 100.0;
    let total_div = max_dist as i64 * 2;
    let min_dist = 1.0;

    let mut pos = None;
    for i in 0..total_div {
      let div_f32 = total_div as f32 - 1.0;
      let dist = (max_dist / div_f32) * i as f32;
      if dist < min_dist {
        continue;
      }

      let p = RayUtils::get_normal_point_with_scale(
        [t.x, t.y, t.z], [f.x, f.y, f.z], dist, voxel_scale
      );

      let p_i64 = [p[0] as i64, p[1] as i64, p[2] as i64];
      let res = self.chunk_manager.get_voxel_safe(&p_i64);

      if res.is_some() && res.unwrap() != 0 {
        pos = Some(Vec3::new(p[0], p[1], p[2]));
        // println!("p {:?}", pos);
        break;
      }
    }

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


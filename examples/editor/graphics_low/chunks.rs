use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_voxel::{BevyVoxelResource, Chunks, MeshComponent, Center};
use utils::Utils;
use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(add)
      .add_system(remove1);
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  mut chunk_query: Query<(Entity, &mut MeshComponent), Changed<MeshComponent>>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {

  for (_, mut mesh_comp) in &mut chunk_query {

    // println!("added {}", mesh_comp.added.iter().len());

    for (data, collider_handle) in mesh_comp.added.iter() {
      // println!("data.indices.len() {}", data.indices.len());

      'graphics: for (entity, graphics) in &chunk_graphics {
        if graphics.key == data.key && graphics.lod == data.lod {
          commands.entity(entity).despawn();

          if graphics.lod == 0 {
            bevy_voxel_res.physics.remove_collider(graphics.collider);
            // println!("remove collider 1");
          }
          // continue 'graphics;
        }

        if graphics.key == data.key {
          if graphics.lod == 0 {
            bevy_voxel_res.physics.remove_collider(graphics.collider);
            // println!("remove collider 1");
          }
        }
      }

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      let mesh_handle = meshes.add(render_mesh);
      let mut pos = bevy_voxel_res.get_pos(data.key);

      let mut color = Color::rgba(0.7, 0.7, 0.7, 0.5);
      if data.lod == 1 {
        color = Color::rgba(0.0, 0.0, 1.0, 0.5);
      }

      let mat = materials.add(color.into());
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: mat,
          transform: Transform::from_translation(pos),
          ..default()
        })
        .insert(ChunkGraphics { 
          key: data.key, 
          lod: data.lod as usize,
          collider: *collider_handle,
        });

      // println!("data.lod {}", data.lod);
    }
    mesh_comp.added.clear();
  }
}

fn remove1(
  mut commands: Commands,

  chunk_graphics: Query<(Entity, &ChunkGraphics)>,
  mesh_comps: Query<(&Center, &MeshComponent)>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
  mut local_res: ResMut<LocalResource>,
) {

  let mut total = 0;
  for (_, g) in &chunk_graphics {
    if g.lod == 1 {
      total += 1;
    }
  }
  if local_res.lod1_total_keys != total {
    local_res.lod1_total_keys = total;
    println!("lod1.len() {}", local_res.lod1_total_keys);
  }
  
  /*
    Detect keys to delete
   */
  // let mut remove_keys = 

  let max_lod = bevy_voxel_res.chunk_manager.depth as usize;
  for (center, mesh_comp) in &mesh_comps {
    // println!("changed12");
    for (entity, graphics) in &chunk_graphics {

      // for lod in 0..max_lod {
      //   if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, lod) {
      //     if graphics.lod != lod {
      //       // If there is duplicate, remove the chunk
      //       for (_, g2) in &chunk_graphics {
      //         if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, lod) {
      //           if g2.lod == lod && graphics.key == g2.key {
      //             commands.entity(entity).despawn_recursive();

      //             println!("Despawned");
      //           }
      //         }
      //       }
      //     }
      //   }
      // }




      for lod in 0..max_lod {
        // println!("lod {}", lod);
        if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, lod) {
          if graphics.lod != lod {
            for (_, g2) in &chunk_graphics {
              if bevy_voxel_res.in_range_by_lod(&center.key, &g2.key, lod) {
                if g2.lod == lod && graphics.key == g2.key {
                  commands.entity(entity).despawn_recursive();
                }
              }
            }
          }
        }
      }

      let r = bevy_voxel_res.ranges.clone();
      let max_range = r[r.len() - 1] as i64;
      if !Utils::in_range(&center.key, &graphics.key, max_range) {
        commands.entity(entity).despawn_recursive();
      }




      // if !Utils::in_range(&center.key, &graphics.key, bevy_voxel_res.ranges[2] as i64) {
      //   commands.entity(entity).despawn_recursive();
      //   // println!("disposed outside");
      // } 


      // println!("dispose---------");
      // if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, 0) {
      //   if graphics.lod != 0 {
      //     for (_, g2) in &chunk_graphics {
      //       if bevy_voxel_res.in_range_by_lod(&center.key, &g2.key, 0) {
      //         if g2.lod == 0 && graphics.key == g2.key {
      //           commands.entity(entity).despawn_recursive();

      //           // println!("dispose lod0 {:?}", g2.key);
      //         }
      //       }
      //     }
      //   }
      // }

      // if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, 1) {
      //   if graphics.lod != 1 {
      //     for (_, g2) in &chunk_graphics {
      //       if bevy_voxel_res.in_range_by_lod(&center.key, &g2.key, 1) {
      //         if g2.lod == 1 && graphics.key == g2.key {
      //           commands.entity(entity).despawn_recursive();

      //           // println!("dispose lod1 {:?}", g2.key);
      //         }
      //       }
      //     }
      //   }
      // }

      // println!("bevy_voxel_res.ranges[2] {}", bevy_voxel_res.ranges[2]);
      // if !Utils::in_range(&center.key, &graphics.key, bevy_voxel_res.ranges[2] as i64) {
      //   commands.entity(entity).despawn_recursive();
      // }

      // if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, 1) {
      //   if graphics.lod != 1 {
      //     for (_, g2) in &chunk_graphics {
      //       if bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, 1) {
      //         if g2.lod == 1 && graphics.key == g2.key {
      //           commands.entity(entity).despawn_recursive();
      //         }
      //       }
      //     }
      //   }
      // }


      


      // if graphics.lod == 0 &&
      // !bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, graphics.lod) {
      //   bevy_voxel_res.physics.remove_collider(graphics.collider);
      // }

      // if graphics.lod == max_lod {
      //   if !bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, graphics.lod) {
      //     commands.entity(entity).despawn_recursive();
      //   }
      // }
    }
  }
}




/* fn remove(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Center)>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {

  let ranges = bevy_voxel_res.ranges.clone();
  for (_, center) in &chunk_query {
    for (entity, graphics) in &chunk_graphics {

      if !bevy_voxel_res.in_range_by_lod(&center.key, &graphics.key, graphics.lod) {
        commands.entity(entity).despawn_recursive();

        if graphics.lod == 0 {
          bevy_voxel_res.physics.remove_collider(graphics.collider);
          // println!("remove collider 2");
        }
      }
      
    }
  }
} */

#[derive(Resource)]
struct LocalResource {
  total_keys: usize,  // For testing
  total_mesh: usize,  // For testing

  lod1_total_keys: usize,
}

impl Default for LocalResource {
  fn default() -> Self {
    Self {
      total_keys: 0,
      total_mesh: 0,
      lod1_total_keys: 0,
    }
  }
}


#[cfg(test)]
mod tests {
  use bevy_voxel::BevyVoxelResource;
  use rapier3d::prelude::ColliderHandle;
  use crate::graphics::ChunkGraphics;


  #[test]
  fn test_duplicate() -> Result<(), String> {
    let res = BevyVoxelResource::default();

    let center_key = [0, 0, 0];
    let g1 = ChunkGraphics {
      key: [1, 0, 0],
      lod: 0,
      collider: ColliderHandle::invalid(),
    };

    let g2 = ChunkGraphics {
      key: [1, 0, 0],
      lod: 1,
      collider: ColliderHandle::invalid(),
    };

    if res.in_range_by_lod(&center_key, &g2.key, 0) {
      if g2.lod != 0 {
        if res.in_range_by_lod(&center_key, &g1.key, 0) {
          if g1.lod == 0 && g2.key == g1.key {
            println!("Duplicate");
          }
        }
      }
    }

    Ok(())
  }

}



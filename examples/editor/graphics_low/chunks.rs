use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_voxel::{BevyVoxelResource, Chunks, MeshComponent, Center};
use utils::Utils;
use crate::graphics::ChunkGraphics;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(LocalResource::default())
      .add_system(add);
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
    for (data, collider_handle) in mesh_comp.added.iter() {
      'graphics: for (entity, graphics) in &chunk_graphics {
        if graphics.key == data.key && graphics.lod == data.lod {
          commands.entity(entity).despawn();
        }

        if graphics.key == data.key {
          if graphics.lod == 0 {
            bevy_voxel_res.physics.remove_collider(graphics.collider);
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



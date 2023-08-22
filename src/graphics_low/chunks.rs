use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use bevy_voxel::BevyVoxelResource;
use voxels::{utils::key_to_world_coord_f32, chunk::{adjacent_keys, adj_keys_by_scale}};
use crate::{data::{GameResource}, components::{chunk::Chunks, player::Player}, graphics::{ChunkGraphics}};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add)
      // .add_system(remove)
      ;
  }
}

fn add(
  game_res: Res<GameResource>,

  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,

  chunk_query: Query<(Entity, &Chunks), Changed<Chunks>>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {

  for (_, chunks) in &chunk_query {
    for (entity, graphics) in &chunk_graphics {
      commands.entity(entity).despawn_recursive();
    }

    for mesh in &chunks.data {
      let data = &mesh.data;

      let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions.clone());
      render_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals.clone());
      render_mesh.set_indices(Some(Indices::U32(data.indices.clone())));

      let mesh_handle = meshes.add(render_mesh);
      let mut pos_f32 = bevy_voxel_res.get_pos(mesh.key);


      let mat = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
      commands
        .spawn(MaterialMeshBundle {
          mesh: mesh_handle,
          material: mat,
          transform: Transform::from_xyz(pos_f32[0], pos_f32[1], pos_f32[2]),
          ..default()
        })
        .insert(ChunkGraphics { key: mesh.key.clone() });
    }
  }
}

fn remove(
  mut commands: Commands,
  chunk_graphics: Query<(Entity, &ChunkGraphics)>,
  chunk_query: Query<(Entity, &Chunks, &Player), Changed<Chunks>>,
  game_res: Res<GameResource>,
) {
  for (_, _chunks, player) in &chunk_query {
    let scale = game_res.voxel_scale;
    let keys = adj_keys_by_scale(player.key, 1, scale);

    for (entity, graphics) in &chunk_graphics {
      if !keys.contains(&graphics.key) {
        commands.entity(entity).despawn_recursive();
      }
    }
  }
}


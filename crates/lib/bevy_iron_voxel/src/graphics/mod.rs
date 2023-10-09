use std::f32::consts::PI;

use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use bevy_voxel::{MeshComponent, BevyVoxelResource, Center};
use rapier3d::prelude::ColliderHandle;
use utils::Utils;

pub mod chunk_preview;
mod player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GraphicsResource::default())
      .add_plugins(player::CustomPlugin)
      .add_plugins(chunk_preview::CustomPlugin)
      .add_systems(Startup, startup)
      .add_systems(Update, toggle_showhide)
      .add_systems(Update, remove);
  }
}

fn startup(
  mut commands: Commands, 
) {
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 600.0,
      ..Default::default()
    },
    transform: Transform::from_xyz(0.0, 5.0, 0.0),
    ..Default::default()
  });

  commands.spawn(DirectionalLightBundle {
    directional_light: DirectionalLight {
      illuminance: 10000.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform {
      translation: Vec3::new(0.0, 2.0, 0.0),
      rotation: Quat::from_rotation_x(-PI / 4.),
      ..default()
    },
    // The default cascade config is designed to handle large scenes.
    // As this example has a much smaller world, we can tighten the shadow
    // bounds for better visual quality.
    cascade_shadow_config: CascadeShadowConfigBuilder {
      first_cascade_far_bound: 4.0,
      maximum_distance: 10.0,
      ..default()
    }
    .into(),
    ..default()
  });
}

fn toggle_showhide(
  _key_input: Res<Input<KeyCode>>,
  mut previews: Query<(&mut Visibility, &ChunkPreviewGraphics)>,
  graphics_res: Res<GraphicsResource>,
) {
  // if key_input.just_pressed(KeyCode::P) {
  //   graphics_res.show_preview = !graphics_res.show_preview;

  //   info!("graphics_res.show_preview {}", graphics_res.show_preview);
  // }

  for (mut visibility, _preview) in &mut previews {

    if !graphics_res.show_preview {
      *visibility = Visibility::Hidden;
    }

    if graphics_res.show_preview {
      *visibility = Visibility::Visible;
    }
  }
}

fn remove(
  mut commands: Commands,

  chunk_graphics: Query<(Entity, &ChunkGraphics)>,
  mesh_comps: Query<(&Center, &MeshComponent)>,
  bevy_voxel_res: Res<BevyVoxelResource>,
) {
  let max_lod = bevy_voxel_res.chunk_manager.depth as usize;
  for (center, _mesh_comp) in &mesh_comps {
    for (entity, graphics) in &chunk_graphics {
      for lod in 0..max_lod {
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
    }
  }
}

#[derive(Resource)]
pub struct GraphicsResource {
  pub show_preview: bool,
}

impl Default for GraphicsResource {
  fn default() -> Self {
    Self {
      show_preview: true,
    }
  }
}



#[derive(Component)]
pub struct ChunkGraphics {
  pub key: [i64; 3],
  pub lod: usize,
  pub collider: ColliderHandle,
}

#[derive(Component, Clone)]
pub struct ChunkPreviewGraphics {
  pub parent: Entity,
}
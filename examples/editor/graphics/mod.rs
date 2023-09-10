use std::f32::consts::PI;

use bevy::{prelude::*, pbr::CascadeShadowConfigBuilder};
use rapier3d::prelude::ColliderHandle;

pub mod chunk_preview;
mod player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GraphicsResource::default())
      .add_plugin(player::CustomPlugin)
      .add_plugin(chunk_preview::CustomPlugin)
      .add_startup_system(startup)
      .add_system(toggle_showhide);
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
  key_input: Res<Input<KeyCode>>,
  mut previews: Query<(&mut Visibility, &ChunkPreviewGraphics)>,
  mut graphics_res: ResMut<GraphicsResource>,
) {
  if key_input.just_pressed(KeyCode::P) {
    graphics_res.show_preview = !graphics_res.show_preview;

    info!("graphics_res.show_preview {}", graphics_res.show_preview);
  }

  for (mut visibility, _preview) in &mut previews {

    if !graphics_res.show_preview {
      *visibility = Visibility::Hidden;
    }

    if graphics_res.show_preview {
      *visibility = Visibility::Visible;
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
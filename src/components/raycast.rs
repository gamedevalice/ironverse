use bevy::prelude::*;
use bevy_flycam::FlyCam;
use rapier3d::{prelude::{Vector, QueryFilter, Ray}, na::Point3};
use crate::{physics::Physics, data::GameResource};
use super::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add)
      .add_system(update);
  }
}

fn add(
  mut commands: Commands,
  player_query: Query<Entity, Added<Player>>,
) {
  for entity in &player_query {
    commands
      .entity(entity)
      .insert(Raycast::default());
  }
}

fn update(
  physics: Res<Physics>,
  game_res: Res<GameResource>,
  mut query: Query<(&Transform, &mut Raycast), With<FlyCam>>,
) {
  for (trans, mut raycast) in query.iter_mut() {
    let look_at = trans.forward();
    // info!("{:?}", look_at);

    let start_pos = trans.translation + raycast.adj;
    let dir = look_at.clone();
    let ray = Ray::new(
      Point3::new(start_pos.x, start_pos.y, start_pos.z), 
      Vector::new(dir.x, dir.y, dir.z)
    );
    let max_toi = f32::MAX;
    let solid = true;
    let filter = QueryFilter::only_fixed();

    if let Some((_handle, toi)) = physics.query_pipeline.cast_ray(
      &physics.rigid_body_set, 
      &physics.collider_set, 
      &ray, 
      max_toi, 
      solid, 
      filter
    ) {
      let hit_point = ray.point_at(toi);

      let point = Vec3::new(
        hit_point[0].round(), 
        hit_point[1].round(), 
        hit_point[2].round()
      );
      if raycast.point != point {
        raycast.point = point;
      }
    }
  }
}


#[derive(Component)]
pub struct Raycast {
  pub point: Vec3,
  pub adj: Vec3,
}

impl Default for Raycast {
  fn default() -> Self {
    Self {
      point: Vec3::new(f32::NAN, f32::NAN, f32::NAN),
      adj: Vec3::new(0.0, 0.2, 0.0),
    }
  }
}
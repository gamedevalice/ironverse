use bevy::prelude::*;
use bevy_flycam::FlyCam;
use crate::utils::Math;

use super::GameState;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    // app
    //   .add_system(enter.in_schedule(OnEnter(GameState::Play)))
    //   .add_system(update.in_set(OnUpdate(GameState::Play)))
    //   ;
  }
}

fn enter() {

}

fn update(mut query: Query<&mut Transform, With<FlyCam>>,) {
  for mut transform in query.iter_mut() {
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    let look_at = Math::rot_to_look_at(Vec3::new(pitch, yaw, 0.0));
    // info!("{:?}", look_at);
  }
}




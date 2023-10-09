use bevy::prelude::*;
use bevy_flycam::FlyCam;
use crate::components::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, add);
  }
}

fn add(
  mut commands: Commands,
  query: Query<Entity, Added<Player>>,
) {
  for entity in &query {
    info!("Add cam");

    // let rigid_body = &mut physics.rigid_body_set[player.body];
    // let pos = rigid_body.position().translation;
    let pos = Vec3::new(0.0, 1.59, 0.0);
    let forward = Vec3::new(0.69, -0.15, 0.70);
    commands
      .entity(entity)
      .insert((
        Camera3dBundle {
          transform: Transform::from_translation(pos).looking_to(forward, Vec3::Y),
          ..Default::default()
        },
        FlyCam,
      ));
  }
}









use bevy::prelude::*;
use bevy_flycam::FlyCam;
use crate::components::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add);
  }
}

fn add(
  mut commands: Commands,
  query: Query<Entity, Added<Player>>,
) {
  for entity in &query {
    info!("Add cam");
    commands
      .entity(entity)
      .insert((
        Camera3dBundle {
          transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_to(Vec3::Z, Vec3::Y),
          ..Default::default()
        },
        FlyCam,
      ));
  }
}









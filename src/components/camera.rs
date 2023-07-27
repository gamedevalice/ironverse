use bevy::prelude::*;
use bevy_flycam::FlyCam;
use crate::{components::player::Player, physics::Physics};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add);
  }
}

fn add(
  mut commands: Commands,
  query: Query<(Entity, &Player), Added<Player>>,
  mut physics: ResMut<Physics>,
) {
  for (entity, player) in &query {
    info!("Add cam");

    let rigid_body = &mut physics.rigid_body_set[player.body];
    let pos = rigid_body.position().translation;
    commands
      .entity(entity)
      .insert((
        Camera3dBundle {
          transform: Transform::from_xyz(pos.x, pos.y, pos.z).looking_to(Vec3::Z, Vec3::Y),
          ..Default::default()
        },
        FlyCam,
      ));
  }
}









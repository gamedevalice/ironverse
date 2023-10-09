use bevy::prelude::*;
use crate::components::player::Player;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, add)
      .add_systems(Update, follow_light);
  }
}

fn add(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  players: Query<Entity, Added<Player>>,
) {
  for entity in &players {
    commands
      .entity(entity)
      .insert(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
      });
  }
}

fn follow_light(
  mut light_query: Query<&mut Transform, With<PointLight>>,
  player: Query<&GlobalTransform, With<Player>>,
) {
  for mut tfm in light_query.iter_mut() {
    for global in &player {
      tfm.translation = global.translation();
    }
  }
}

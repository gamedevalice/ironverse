use bevy::prelude::*;
use bevy_flycam::FlyCam;

use crate::utils::Math;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(setup)
      .add_system(test_rotation);
  }
}

fn setup(
  mut commands: Commands,
  mut query: Query<&mut Transform, Added<FlyCam>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for mut trans in &mut query {

    let yaw = std::f32::consts::TAU * 0.5;
    // trans.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, yaw, 0.0);

    // *trans = Transform::from_xyz(0.0, 8.0, -25.0).looking_to(Vec3::Z, Vec3::Y);
    // *trans = Transform::from_xyz(0.0, 1.0, 1.0).looking_to(Vec3::Z, Vec3::Y);


    commands.spawn(PbrBundle {
      mesh: meshes.add(shape::Plane::from_size(5.0).into()),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..default()
    });


    let dist = 10.0;
    commands.spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
      transform: Transform::from_xyz(dist, 0.0, 0.0),
      ..default()
    });

    commands.spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
      transform: Transform::from_xyz(0.0, dist, 0.0),
      ..default()
    });

    commands.spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
      transform: Transform::from_xyz(0.0, 0.0, dist),
      ..default()
    });
  }
}


fn test_rotation(
  mut commands: Commands,
  mut cam_query: Query<(&mut Transform), With<FlyCam>>,
) {
  for mut trans in &mut cam_query {
    let (pitch, yaw, roll) = trans.rotation.to_euler(EulerRot::XYZ);

    // info!("{}: {}: {:?}", pitch, yaw, trans.translation);

    // let look_at = Math::rot_to_look_at(Vec3::new(pitch, yaw, 0.0));
    // info!("{:?}", look_at);
  }
}

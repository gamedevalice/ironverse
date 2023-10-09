use bevy::prelude::*;

// pub mod raycast;
// mod camera;
mod text;
// mod chunks;

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(text::CustomPlugin)
      // .add_startup_system(startup)
      // .add_plugin(LogDiagnosticsPlugin::default())
      // .add_plugin(raycast::CustomPlugin)
      // .add_plugin(camera::CustomPlugin)
      // .add_plugin(chunks::CustomPlugin)
      ;
  }
}

/* fn startup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // plane
  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    ..default()
  });
} */
use bevy::prelude::*;
use bevy_flycam::FlyCam;

use crate::{components::{raycast::Raycast}, utils::Math, data::Player};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_system(add)
      // .add_system(update)
      .add_system(update.in_base_set(CoreSet::PostUpdate))
      ;
  }
}

fn add(
  mut commands: Commands,
  player_query: Query<(Entity, &Player), Added<RaycastDebugger>>,

  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (entity, player) in &player_query {


    info!("Add raycast debugger");
    commands
      .spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.0).into()),
        transform: Transform::from_translation(Vec3::ZERO),
        visibility: Visibility::Visible,
        ..Default::default()
      })
      .with_children(|parent| {
        let height = 200.0;
        let mut transform = Transform::from_scale(Vec3::new(0.1, height, 0.1));
        transform.translation = Vec3::new(0.0, height * 0.5, 0.0);

        parent
          .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.3).into()),
            transform: transform,
            ..Default::default()
          });
      })
      .insert(Debugger {});

    // commands
    //   .entity(entity)
    //   .with_children(|parent| {

    //     let height = 10.0;
    //     let mut transform = Transform::from_xyz(0.0, 0.0, height * -0.5)
    //       .with_scale(Vec3::new(0.1, 0.1, height))
    //       ;
    //     // let mut transform = Transform::from_scale(Vec3::new(0.1, height, 0.1));
    //     // transform.translation = Vec3::new(0.0, height * 0.5, 0.0);
    //     // transform.rotation = Quat::from_euler(
    //     //   EulerRot::XYZ, std::f32::consts::PI * 0.1, 0.0, 0.0
    //     // );

    //     parent
    //       .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgba(0.0, 0.0, 0.6, 0.3).into()),
    //         transform: transform,
    //         ..Default::default()
    //       })
    //       .insert(Debugger {});
    //   });
  }
}

// fn update1(
//   mut raycast: Query<(&mut Transform, &Debugger)>,
// ) {

//   for (mut ray_trans, raycast) in &mut raycast {
//     ray_trans.rotation *= Quat::from_rotation_y(std::f32::consts::PI * 0.01);
//   }
// }

fn update(
  mut debugger_ray: Query<(&mut Transform, &Debugger)>,
  mut cam_query: Query<(&GlobalTransform, &Raycast), With<FlyCam>>,
) {
  for (cam_trans, ray) in &cam_query {
    for (mut ray_trans, raycast) in &mut debugger_ray {
      let trans = cam_trans.compute_transform();
      let look_at = trans.forward();

      let pos = trans.translation + ray.adj;
      // let pos = trans.translation + Vec3::new(5.0, 0.0, 0.0);
      // let pos = Vec3::new(0.0, 0.0, 0.0);
      let t = Transform::from_xyz(pos.x, pos.y, pos.z).looking_to(look_at, Vec3::Y);
      
      *ray_trans = t;
      ray_trans.rotation *= Quat::from_rotation_x(std::f32::consts::PI * -0.5);

      // info!("ray_trans.rotation {:?}", ray_trans.rotation);
    }
  }
}


#[derive(Component)]
struct Debugger {

}


#[derive(Component)]
pub struct RaycastDebugger;

impl Default for RaycastDebugger {
  fn default() -> Self {
    Self { }
  }
}
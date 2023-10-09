use bevy::prelude::*;
use bevy_voxel::{BevyVoxelResource, Center};
use rapier3d::{na::Vector3, prelude::{RigidBodyHandle, ColliderHandle}};
use crate::data::{GameResource, GameState};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(OnEnter(GameState::Start), start)
      .add_systems(OnEnter(GameState::Init), init)
      .add_systems(Update, update)
      ;
  }
}

fn start(
  mut commands: Commands,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  // // let pos = [0.0, 5.0, 0.0];
  let pos = Vec3::new(0.0, 0.4, 0.0);

  let (body, collider) = bevy_voxel_res.physics.spawn_character(1.0, 0.5, pos);
  let k = bevy_voxel_res.get_key(pos);

  println!("start player key {:?}", k);
  commands
    .spawn(
      (Player::new(body, collider, k),
      Center {prev_key: k, key: k } )
    );
}

fn init(
  mut commands: Commands,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
  game_res: Res<GameResource>,
) {
  let p = game_res.data.status.position;
  let pos = Vec3::new(p[0], p[1], p[2]);
  let (body, collider) = bevy_voxel_res.physics.spawn_character(
    1.0, 0.5, pos
  );
  let k = bevy_voxel_res.get_key(pos);
  commands
    .spawn(
      (Player::new(body, collider, k),
      Center {prev_key: k, key: k } )
    );

  // info!("player init() {:?}", pos);
}

pub fn create_new_player(mut bevy_voxel_res: ResMut<BevyVoxelResource>, game_res: &Res<GameResource>) -> (Player, Center) {

  let p = game_res.data.status.position;
  let pos = Vec3::new(p[0], p[1], p[2]);
  let (body, collider) = bevy_voxel_res.physics.spawn_character(
    1.0, 0.5, pos
  );
  let k = bevy_voxel_res.get_key(pos);
  (
    Player::new(body, collider, k),
    Center {prev_key: k, key: k } 
  )
}

fn update(
  mut query: Query<(&Transform, &mut Center, &mut Player)>,
  mut bevy_voxel_res: ResMut<BevyVoxelResource>,
) {
  for (trans, mut center, mut player) in &mut query {
    let p = trans.translation;
    let rigid_body = &mut bevy_voxel_res.physics.rigid_body_set[player.body];
    rigid_body.set_position(Vector3::new(p.x, p.y, p.z).into(), false);

    let k = bevy_voxel_res.get_key(p);
    if player.key != k {
      player.prev_key = player.key.clone();
      player.key = k;

      center.prev_key = center.key;
      center.key = k;
    }
  }
}

#[derive(Component, Debug, Clone)]
pub struct Player {
  pub body: RigidBodyHandle,
  pub collider: ColliderHandle,
  pub prev_key: [i64; 3],
  pub key: [i64; 3],
}

impl Player {
  pub fn new(b: RigidBodyHandle, c: ColliderHandle, k: [i64; 3]) -> Self {
    
    Self {
      body: b,
      collider: c,
      prev_key: k.clone(),
      key: k
    }
  }
}
use bevy::prelude::*;
use rapier3d::{prelude::{RigidBodySet, ColliderSet, PhysicsPipeline, ColliderBuilder, RigidBodyBuilder, Real, Vector, IntegrationParameters, IslandManager, MultibodyJointSet, ImpulseJointSet, NarrowPhase, BroadPhase, CCDSolver, RigidBodyHandle, Collider, ColliderHandle, InteractionGroups, QueryPipeline, Group}, na::Vector3};

pub struct CustomPlugin;
impl Plugin for CustomPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Physics::default())
      .add_systems(Update, update);
  }
}


fn update(mut physics: ResMut<Physics>) {
  physics.step();
}


#[derive(Resource)]
pub struct Physics {
  pub pipeline: PhysicsPipeline,
  pub query_pipeline: QueryPipeline,
  pub rigid_body_set: RigidBodySet,
  pub collider_set: ColliderSet,
  pub gravity: Vector<Real>,
  pub integration_parameters: IntegrationParameters,
  pub island_manager: IslandManager,
  pub broad_phase: BroadPhase,
  pub narrow_phase: NarrowPhase,
  pub impulse_joint_set: ImpulseJointSet,
  pub multibody_joint_set: MultibodyJointSet,
  pub ccd_solver: CCDSolver,
}

impl Default for Physics {
  fn default() -> Self {
    Self {
      pipeline: PhysicsPipeline::new(),
      query_pipeline: QueryPipeline::new(),
      rigid_body_set: RigidBodySet::new(),
      collider_set: ColliderSet::new(),
      gravity: Vector::y() * -9.81,
      integration_parameters: IntegrationParameters {
        dt: 1.0 / 30.0,
        ..default()
      },
      island_manager: IslandManager::new(),
      broad_phase: BroadPhase::new(),
      narrow_phase: NarrowPhase::new(),
      impulse_joint_set: ImpulseJointSet::new(),
      multibody_joint_set: MultibodyJointSet::new(),
      ccd_solver: CCDSolver::new(),
    }
  }
}

impl Physics {
  pub fn step(&mut self) {
    let physics_hooks = ();
    let event_handler = ();

    self.pipeline.step(
      &self.gravity, 
      &self.integration_parameters, 
      &mut self.island_manager, 
      &mut self.broad_phase, 
      &mut self.narrow_phase, 
      &mut self.rigid_body_set, 
      &mut self.collider_set, 
      &mut self.impulse_joint_set, 
      &mut self.multibody_joint_set, 
      &mut self.ccd_solver,
      None,
      &physics_hooks,
      &event_handler
    );

    self.query_pipeline.update(
      &self.rigid_body_set, 
      &self.collider_set
    );
  }

  #[allow(dead_code)]
  pub fn spawn_character(&mut self, depth: f32, radius: f32, pos: Vec3) -> (RigidBodyHandle, ColliderHandle) {
    let collider = ColliderBuilder::capsule_y(depth * 0.5, radius)
      .collision_groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
      // .collision_groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_3))
      .build();
    let rigid_body = RigidBodyBuilder::dynamic()
      .translation(Vector3::from([pos.x, pos.y, pos.z]))
      // .lock_rotations()
      .gravity_scale(0.0)
      .linear_damping(5.0)
      .build();
    let body_handle = self.rigid_body_set.insert(rigid_body);
    let collider_handle = self.insert_with_parent(collider, body_handle.clone());
    (body_handle, collider_handle)
  }

  #[allow(dead_code)]
  pub fn insert_with_parent(&mut self, collider: Collider, handle: RigidBodyHandle) -> ColliderHandle {
    self
      .collider_set
      .insert_with_parent(collider, handle, &mut self.rigid_body_set)
  }

  #[allow(dead_code)]
  pub fn remove_collider(&mut self, handle: ColliderHandle) {
    self
      .collider_set
      .remove(handle, &mut self.island_manager, &mut self.rigid_body_set, true);
  }
}

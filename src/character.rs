use bevy::prelude::*;
use bevy_rapier3d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    dynamics::RigidBody,
};

#[derive(Component)]
pub struct NameComponent(pub String);

#[derive(Component)]
pub struct HealthComponent(pub f32);

#[derive(Bundle)]
pub struct CharacterPhysicsBody {
    pub rigid_body: RigidBody,
    pub kinematic_controller: KinematicCharacterController,
    pub kinematic_controller_output: KinematicCharacterControllerOutput,
}

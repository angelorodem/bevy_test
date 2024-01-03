use bevy::prelude::*;
use bevy_rapier3d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    dynamics::{Ccd, Damping, LockedAxes, RigidBody},
    geometry::{Collider, Friction},
};
use std::collections::VecDeque;

use crate::movable::MovableBundle;

#[derive(Component)]
pub struct NameComponent(pub String);

#[derive(Component)]
pub struct HealthComponent(pub f32);

#[derive(Bundle)]
pub struct ControllableCharacterPhysicsBody {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub kinematic_controller: KinematicCharacterController,
    pub kinematic_controller_output: KinematicCharacterControllerOutput,
    pub ccd: Ccd,
    pub name: NameComponent,
    pub health: HealthComponent,
    pub movable_bundle: MovableBundle,
    pub model: SceneBundle,
}

#[derive(Bundle)]
pub struct CharacterPhysicBody {
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub damping: Damping,
    pub friction: Friction,
}

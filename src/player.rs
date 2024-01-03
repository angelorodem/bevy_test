use std::time::Duration;

use crate::asset_loader::PlayerSceneAssets;
use crate::character::{ControllableCharacterPhysicsBody, HealthComponent, NameComponent};
use crate::movable::{AnimatedCharacterMovable, Movable, MovableBundle};
use crate::states::GameState;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::math::vec3;
use bevy::prelude::*;

use bevy_rapier3d::control::{
    CharacterAutostep, CharacterLength, KinematicCharacterController,
    KinematicCharacterControllerOutput,
};
use bevy_rapier3d::dynamics::{Ccd, Velocity};
use bevy_rapier3d::geometry::{ActiveCollisionTypes, ComputedColliderShape, Friction};
use bevy_rapier3d::pipeline::{CollisionEvent, ContactForceEvent};
use bevy_rapier3d::{
    dynamics::{LockedAxes, RigidBody, Sleeping},
    geometry::{Collider, ColliderMassProperties},
};

#[derive(Component)]
pub struct PlayerTag;

#[derive(Bundle)]
pub struct PlayerBundle {
    character_physics_body: ControllableCharacterPhysicsBody,
    tag: PlayerTag,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player_command)
            .add_systems(
                Update,
                (move_player, display_events).run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player_command(
    mut commands: Commands,
    asset_server: ResMut<PlayerSceneAssets>,
    mut meshes: ResMut<Assets<Mesh>>,                // example plane
    mut materials: ResMut<Assets<StandardMaterial>>, // example plane
) {
    spawn_player(&mut commands, &asset_server, "Player".to_string());
}

fn spawn_player(commands: &mut Commands, asset_server: &ResMut<PlayerSceneAssets>, name: String) {
    commands
        .spawn(PlayerBundle {
            character_physics_body: ControllableCharacterPhysicsBody {
                ccd: Ccd::enabled(),
                rigid_body: RigidBody::KinematicPositionBased,
                collider: Collider::capsule(vec3(0.0, 0.8, 0.0), vec3(0.0, 2., 0.0), 0.7),
                kinematic_controller: KinematicCharacterController {
                    offset: CharacterLength::Absolute(0.2),
                    apply_impulse_to_dynamic_bodies: true,
                    snap_to_ground: Some(CharacterLength::Relative(0.5)),
                    autostep: Some(CharacterAutostep {
                        max_height: CharacterLength::Relative(1.),
                        min_width: CharacterLength::Relative(1.),
                        include_dynamic_bodies: false,
                    }),
                    max_slope_climb_angle: (45.0 as f32).to_radians(),
                    min_slope_slide_angle: (30.0 as f32).to_radians(),
                    ..Default::default()
                },
                kinematic_controller_output: KinematicCharacterControllerOutput::default(),
                model: SceneBundle {
                    scene: asset_server.player.clone(),
                    transform: Transform::from_translation(Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    }),
                    ..Default::default()
                },
                name: NameComponent(name),
                health: HealthComponent(100.0),
                movable_bundle: MovableBundle {
                    movable: Movable {
                        max_speed: 14.0,
                        max_acceleration: 20.0,
                        ..Default::default()
                    },
                    movable_animation: AnimatedCharacterMovable {
                        run_animation: asset_server.player_run_animation.clone(),
                        walk_animation: asset_server.player_walk_animation.clone(),
                        idle_animations: asset_server.player_idle_animations.clone(),
                    },
                },
            },
            tag: PlayerTag,
        })
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC);
}

fn move_player(
    mut player_transforms: Query<(&mut Transform, &mut Movable), With<PlayerTag>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut player_transform, mut player_movable) in player_transforms.iter_mut() {
        let rotation = if keyboard_input.pressed(KeyCode::A) {
            5.0 * time.delta_seconds()
        } else if keyboard_input.pressed(KeyCode::D) {
            -5.0 * time.delta_seconds()
        } else {
            0.0
        };

        let x_movemet = if keyboard_input.pressed(KeyCode::W) {
            9.0
        } else if keyboard_input.pressed(KeyCode::S) {
            -9.0
        } else {
            -(player_movable.speed * 6.0 + player_movable.acceleration)
        };

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            player_movable.fast = true;
        } else {
            player_movable.fast = false;
        }

        player_movable.acceleration = (player_movable.acceleration + x_movemet).clamp(
            -player_movable.max_acceleration,
            player_movable.max_acceleration,
        );
        player_transform.rotate_y(rotation);
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

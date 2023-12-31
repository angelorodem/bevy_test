use std::time::Duration;

use crate::asset_loader::PlayerSceneAssets;
use crate::character::{CharacterPhysicsBody, HealthComponent, NameComponent};
use crate::movable::{AnimatedCharacterMovable, Movable};
use crate::states::GameState;
use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;

use bevy_rapier3d::control::{
    CharacterAutostep, CharacterLength, KinematicCharacterController,
    KinematicCharacterControllerOutput,
};
use bevy_rapier3d::dynamics::Velocity;
use bevy_rapier3d::geometry::{ComputedColliderShape, Friction};
use bevy_rapier3d::{
    dynamics::{LockedAxes, RigidBody, Sleeping},
    geometry::{Collider, ColliderMassProperties},
};

#[derive(Component)]
pub struct PlayerTag;

#[derive(Bundle)]
pub struct PlayerBundle {
    character_physics_body: CharacterPhysicsBody,
    model: SceneBundle,
    name: NameComponent,
    health: HealthComponent,
    tag: PlayerTag,
    movable: Movable,
    movable_animation: AnimatedCharacterMovable,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player_command)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player_command(
    mut commands: Commands,
    asset_server: ResMut<PlayerSceneAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    asset_gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,                // example plane
    mut materials: ResMut<Assets<StandardMaterial>>, // example plane
) {
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(1.0, 4.0, 0.0),
    //     point_light: PointLight {
    //         intensity: 3000.0,
    //         radius: 2.0,
    //         color: Color::rgb(0.2, 0.2, 1.0),
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        sectors: 128,
        stacks: 64,
        ..default()
    }));

    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                emissive: Color::rgb_linear(0.2, 0.2, 10.0),
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 4.0, 0.0).with_scale(Vec3::splat(0.8)),
            ..default()
        })
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 15000.0,
                    radius: 0.8,
                    shadows_enabled: true,
                    color: Color::rgb(0.2, 0.2, 1.0),
                    ..default()
                },
                ..default()
            });
        });

    let player_mesh = meshes
        .get(
            &asset_gltf_meshes
                .get(
                    assets_gltf
                        .get(&asset_server.player_glb)
                        .unwrap()
                        .meshes
                        .first()
                        .unwrap()
                        .clone(),
                )
                .unwrap()
                .primitives
                .first()
                .unwrap()
                .mesh,
        )
        .unwrap();

    spawn_player(
        &mut commands,
        &asset_server,
        &player_mesh,
        "Player".to_string(),
    );

    //.insert(Collider::capsule_y(0.7, 0.6));

    // Test plane
    // commands.spawn(PbrBundle {
    //     transform: Transform::from_xyz(25000.0, 0.0, 0.0),
    //     mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
    //     material: materials.add(.into()),
    //     ..default()
    // });
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(100.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.chess_texture.clone()),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 0.08,
                ..default()
            }),
            ..default()
        })
        .insert((Collider::cuboid(50.0, 0.1, 50.0),))
        .insert(RigidBody::Fixed);

    // test block
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(4.0, 4.0, 4.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.chess_texture.clone()),
                alpha_mode: AlphaMode::Opaque,
                ..default()
            }),
            transform: Transform::from_xyz(-4.0, 2.0, -4.0),
            ..default()
        })
        .insert((Collider::cuboid(2.0, 2.0, 2.0), RigidBody::Fixed));

    // test block2
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(4.0, 1.0, 4.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.chess_texture.clone()),
                alpha_mode: AlphaMode::Opaque,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 0.5, -4.0),
            ..default()
        })
        .insert((Collider::cuboid(2.0, 0.5, 2.0), RigidBody::Fixed));
}

fn spawn_player(
    mut commands: &mut Commands,
    asset_server: &ResMut<PlayerSceneAssets>,
    player_mesh: &Mesh,
    name: String,
) {
    commands
        .spawn(PlayerBundle {
            character_physics_body: CharacterPhysicsBody {
                rigid_body: RigidBody::KinematicPositionBased,
                kinematic_controller: KinematicCharacterController {
                    offset: CharacterLength::Absolute(0.1),
                    apply_impulse_to_dynamic_bodies: true,
                    snap_to_ground: Some(CharacterLength::Absolute(10.)),
                    autostep: Some(CharacterAutostep {
                        max_height: CharacterLength::Relative(1.),
                        min_width: CharacterLength::Relative(0.6),
                        include_dynamic_bodies: false,
                    }),
                    ..Default::default()
                },
                kinematic_controller_output: KinematicCharacterControllerOutput::default(),
            },
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
            tag: PlayerTag,
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
        })
        .insert(Collider::from_bevy_mesh(player_mesh, &ComputedColliderShape::ConvexHull).unwrap())
        // Position the collider relative to the rigid-body.
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 1.4, 0.0)));
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

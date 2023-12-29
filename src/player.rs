use std::time::Duration;

use crate::asset_loader::PlayerSceneAssets;
use crate::movable::{AnimatedCharacterMovable, Movable};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerTag;

#[derive(Component)]
pub struct NameComponent(String);

#[derive(Component)]
pub struct HealthComponent(f32);

#[derive(Bundle)]
pub struct PlayerBundle {
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
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<PlayerSceneAssets>,
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

    // commands
    //     .spawn(PbrBundle {
    //         mesh: mesh.clone(),
    //         material: materials.add(StandardMaterial {
    //             emissive: Color::rgb_linear(0.2, 0.2, 10.0),
    //             ..default()
    //         }),
    //         transform: Transform::from_xyz(1.0, 4.0, 0.0).with_scale(Vec3::splat(0.8)),
    //         ..default()
    //     })
    //     .with_children(|children| {
    //         children.spawn(PointLightBundle {
    //             point_light: PointLight {
    //                 intensity: 15000.0,
    //                 radius: 0.8,
    //                 shadows_enabled: true,
    //                 color: Color::rgb(0.2, 0.2, 1.0),
    //                 ..default()
    //             },
    //             ..default()
    //         });
    //     });

    commands.spawn(PlayerBundle {
        model: SceneBundle {
            scene: asset_server.player.clone(),
            transform: Transform::from_translation(Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        name: NameComponent("Player 1".to_string()),
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
    });

    // Test plane
    // commands.spawn(PbrBundle {
    //     transform: Transform::from_xyz(25000.0, 0.0, 0.0),
    //     mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
    //     material: materials.add(.into()),
    //     ..default()
    // });
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(100.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            perceptual_roughness: 0.08,
            ..default()
        }),
        ..default()
    });
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

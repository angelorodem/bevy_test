use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{LockedAxes, RigidBody, Sleeping, Velocity},
    geometry::{Collider, ColliderMassProperties, Friction},
};

use crate::{
    asset_loader::SkeletonSceneAssets,
    character::{HealthComponent, NameComponent},
    movable::{AnimatedCharacterMovable, Movable},
    player::PlayerTag,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy)
            .add_systems(Update, execute_ai);
    }
}

#[derive(Component)]
pub struct EnemyTag;

#[derive(Component, Default)]
pub enum AiType {
    #[default]
    FOLLOW,
    NONE,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub model: SceneBundle,
    pub name: NameComponent,
    pub health: HealthComponent,
    pub tag: EnemyTag,
    pub movable: Movable,
    pub movable_animation: AnimatedCharacterMovable,
    pub ai_type: AiType,
}

fn spawn_enemy(mut commands: Commands, asset_server: ResMut<SkeletonSceneAssets>) {
    commands.spawn(EnemyBundle {
        model: SceneBundle {
            scene: asset_server.skeleton.clone(),
            transform: Transform::from_translation(Vec3 {
                x: 4.,
                y: 0.0,
                z: 4.,
            }),
            ..Default::default()
        },
        name: NameComponent("Evil boy".to_string()),
        health: HealthComponent(100.0),
        tag: EnemyTag,
        movable: Movable {
            max_speed: 7.0,
            max_acceleration: 20.0,
            ..Default::default()
        },
        movable_animation: AnimatedCharacterMovable {
            run_animation: asset_server.skeleton_run_animation.clone(),
            walk_animation: asset_server.skeleton_walk_animation.clone(),
            idle_animations: asset_server.skeleton_idle_animations.clone(),
        },
        ai_type: AiType::FOLLOW,
    });
}

fn execute_ai(
    mut enemies: Query<(&mut Movable, &mut Transform, &AiType), With<EnemyTag>>,
    player: Query<&Transform, (With<PlayerTag>, Without<EnemyTag>)>,
) {
    let target = player.get_single();
    if let Ok(player_transform) = target {
        for (mut movable, mut enemy_transform, ai_type) in enemies.iter_mut() {
            match ai_type {
                AiType::FOLLOW => {
                    let r_pos = player_transform.translation - enemy_transform.translation;
                    let target = enemy_transform.translation - r_pos;
                    let look_at_target =
                        Vec3::new(target.x, enemy_transform.translation.y, target.z);
                    enemy_transform.look_at(look_at_target, Vec3::Y);

                    let distance = enemy_transform
                        .translation
                        .distance(player_transform.translation);

                    if distance > 7.0 {
                        if distance > 18.0 {
                            movable.acceleration = 10.0;
                            movable.fast = false;
                        } else {
                            movable.fast = true;
                            movable.acceleration = 15.0;
                        }
                    } else {
                        movable.acceleration = -(movable.speed);
                    }
                }
                AiType::NONE => {}
            }
        }
    }
}

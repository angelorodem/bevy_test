use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::{Damping, LockedAxes, RigidBody, Sleeping, Velocity},
    geometry::{Collider, ColliderMassProperties, Friction},
};

use crate::{
    asset_loader::{EnemySceneAssets, SkeletonSceneAssets},
    character::{CharacterPhysicBody, HealthComponent, NameComponent},
    movable::{AnimatedCharacterMovable, Movable},
    player::PlayerTag,
    states::GameState,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Enemies>()
            .add_systems(
                OnEnter(GameState::Playing),
                (load_skeleton_type, load_demon_type, spawn_enemies).chain(),
            )
            .add_systems(Update, execute_ai.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct EnemyTag;

#[derive(Component, Default, Clone, Copy)]
pub enum AiType {
    #[default]
    FOLLOW,
    NONE,
}

#[derive(Default)]
pub struct EnemyTypeBase {
    pub animation: AnimatedCharacterMovable,
    pub scene: Handle<Scene>,
    pub ai_type: AiType,
}

#[derive(Resource, Default)]
pub struct Enemies {
    pub skeleton: EnemyTypeBase,
    pub demon: EnemyTypeBase,
}

#[derive(Bundle)]
pub struct EnemySpawnBundle {
    pub model: SceneBundle,
    pub name: NameComponent,
    pub health: HealthComponent,
    pub tag: EnemyTag,
    pub movable: Movable,
    pub movable_animation: AnimatedCharacterMovable,
    pub ai_type: AiType,
    pub rigid_body: CharacterPhysicBody,
}

fn load_skeleton_type(mut enemies: ResMut<Enemies>, asset_server: ResMut<EnemySceneAssets>) {
    enemies.skeleton.animation = AnimatedCharacterMovable {
        run_animation: asset_server.skeleton.skeleton_run_animation.clone(),
        walk_animation: asset_server.skeleton.skeleton_walk_animation.clone(),
        idle_animations: asset_server.skeleton.skeleton_idle_animations.clone(),
    };
    enemies.skeleton.scene = asset_server.skeleton.skeleton.clone();
    enemies.skeleton.ai_type = AiType::FOLLOW;
}

fn load_demon_type(mut enemies: ResMut<Enemies>, asset_server: ResMut<EnemySceneAssets>) {
    enemies.demon.animation = AnimatedCharacterMovable {
        run_animation: asset_server.demon.demon_run_animation.clone(),
        walk_animation: asset_server.demon.demon_walk_animation.clone(),
        idle_animations: asset_server.demon.demon_idle_animations.clone(),
    };
    enemies.demon.scene = asset_server.demon.demon.clone();
    enemies.demon.ai_type = AiType::FOLLOW;
}

fn spawn_enemies(mut commands: Commands, enemies: Res<Enemies>) {
    for i in 0..5 {
        for j in 0..5 {
            spawn_enemy(
                Vec3 {
                    x: i as f32,
                    y: 0.0,
                    z: j as f32,
                },
                &mut commands,
                &enemies.demon,
            );
        }
    }
}

fn spawn_enemy(position: Vec3, commands: &mut Commands, enemy: &EnemyTypeBase) {
    commands
        .spawn(EnemySpawnBundle {
            model: SceneBundle {
                scene: enemy.scene.clone(),
                transform: Transform::from_translation(position),
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
            movable_animation: enemy.animation.clone(),
            ai_type: enemy.ai_type.clone(),
            rigid_body: CharacterPhysicBody {
                rigid_body: RigidBody::Dynamic,
                damping: Damping {
                    linear_damping: 0.5,
                    angular_damping: 1.0,
                },
                friction: Friction {
                    coefficient: 1.0,
                    combine_rule: bevy_rapier3d::dynamics::CoefficientCombineRule::Average,
                },
                locked_axes: LockedAxes::ROTATION_LOCKED,
            },
        })
        .with_children(|child| {
            child
                .spawn(Collider::capsule_y(0.7, 0.6))
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    0.0, 1.4, 0.0,
                )))
                .insert(ColliderMassProperties::Density(10.0));
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

use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::{
    asset_loader::SkeletonSceneAssets,
    character::{HealthComponent, NameComponent},
    movable::{AnimatedCharacterMovable, Movable},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy);
    }
}

#[derive(Component)]
pub struct EnemyTag;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub model: SceneBundle,
    pub name: NameComponent,
    pub health: HealthComponent,
    pub tag: EnemyTag,
    pub movable: Movable,
    pub movable_animation: AnimatedCharacterMovable,
}

fn spawn_enemy(mut commands: Commands, asset_server: ResMut<SkeletonSceneAssets>) {
    println!("Spawning enemy");
    commands.spawn(EnemyBundle {
        model: SceneBundle {
            scene: asset_server.skeleton.clone(),
            transform: Transform::from_translation(Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }),
            ..Default::default()
        },
        name: NameComponent("Evil boy".to_string()),
        health: HealthComponent(100.0),
        tag: EnemyTag,
        movable: Movable {
            max_speed: 14.0,
            max_acceleration: 20.0,
            ..Default::default()
        },
        movable_animation: AnimatedCharacterMovable {
            run_animation: asset_server.skeleton_run_animation.clone(),
            walk_animation: asset_server.skeleton_walk_animation.clone(),
            idle_animations: asset_server.skeleton_idle_animations.clone(),
        },
    });
}

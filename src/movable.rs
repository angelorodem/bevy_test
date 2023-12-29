use std::time::Duration;

use bevy::prelude::*;

use crate::asset_loader::AnimationEntityLink;

#[derive(Component, Default)]
pub struct Movable {
    pub acceleration: f32,
    pub max_acceleration: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub fast: bool,
}

#[derive(Component)]
pub struct AnimatedCharacterMovable {
    pub run_animation: Handle<AnimationClip>,
    pub walk_animation: Handle<AnimationClip>,
    pub idle_animations: Vec<Handle<AnimationClip>>,
}

pub struct MovablePlugin;
impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_movables)
            .add_systems(Update, animate_movables);
    }
}

fn move_movables(mut movables: Query<(&mut Transform, &mut Movable)>, time: Res<Time>) {
    for (mut movable_tranform, mut movable_data) in movables.iter_mut() {
        if movable_data.fast {
            movable_data.speed = (movable_data.speed
                + (movable_data.acceleration * time.delta_seconds()))
            .clamp(-movable_data.max_speed, movable_data.max_speed);
        } else {
            movable_data.speed = (movable_data.speed
                + (movable_data.acceleration * time.delta_seconds()))
            .clamp(-movable_data.max_speed / 2.0, movable_data.max_speed / 2.0);
        }

        if movable_data.speed < 1.0 && movable_data.acceleration.abs() < 9.0 {
            movable_data.speed = 0.0;
            movable_data.acceleration = 0.0;
        } else {
            let forward = -movable_tranform.forward();
            let move_vector = forward * movable_data.speed * time.delta_seconds();

            movable_tranform.translation += move_vector;
        }
    }
}

fn animate_movables(
    mut animation_players: Query<&mut AnimationPlayer>,
    targets: Query<(&AnimationEntityLink, &Movable, &AnimatedCharacterMovable)>,
) {
    for (target, movable, movable_animation) in targets.iter() {
        let mut animator = animation_players.get_mut(target.0).unwrap();
        if movable.speed == 0.0 && movable.acceleration == 0.0 {
            if !animator.is_playing_clip(&movable_animation.idle_animations[1]) {
                println!("Idle");
                // hack to fix feet position
                animator
                    .start_with_transition(
                        movable_animation.idle_animations[1].clone_weak(),
                        Duration::from_millis(200),
                    )
                    .repeat();
            }
            continue;
        } else if movable.speed <= movable.max_speed / 2.0 + 0.1 {
            if !animator.is_playing_clip(&movable_animation.walk_animation) {
                animator
                    .play_with_transition(
                        movable_animation.walk_animation.clone_weak(),
                        Duration::from_millis(300),
                    )
                    .repeat();
            }
            animator.set_speed(movable.speed / (movable.max_speed / 2.0));
        } else {
            if !animator.is_playing_clip(&movable_animation.run_animation) {
                animator
                    .play_with_transition(
                        movable_animation.run_animation.clone_weak(),
                        Duration::from_millis(300),
                    )
                    .repeat();
            }
            animator.set_speed(movable.speed / movable.max_speed);
        }
    }
}

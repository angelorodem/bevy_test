use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{
    asset_loader::AnimationEntityLink, debug::PlayerSpeedHistory, enemy::EnemyTag,
    player::PlayerTag, states::GameState,
};

#[derive(Bundle)]
pub struct MovableBundle {
    pub movable: Movable,
    pub movable_animation: AnimatedCharacterMovable,
}

#[derive(Component, Default)]
pub struct Movable {
    pub acceleration: f32,
    pub max_acceleration: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub fast: bool,
    pub falling_time: f32,
}

#[derive(Component, Clone, Default)]
pub struct AnimatedCharacterMovable {
    pub run_animation: Handle<AnimationClip>,
    pub walk_animation: Handle<AnimationClip>,
    pub idle_animations: Vec<Handle<AnimationClip>>,
}

pub struct MovablePlugin;
impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_movables_player, move_movables_enemy, animate_movables)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn move_movables_player(
    mut movables: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut Movable,
        ),
        With<PlayerTag>,
    >,
    mut player_history: Option<ResMut<PlayerSpeedHistory>>,
    time: Res<Time>,
) {
    for (movable_tranform, mut controller, controller_output, mut movable_data) in
        movables.iter_mut()
    {
        let mut move_vector = if !controller_output.grounded {
            Vec3::new(0.0, -10.0, 0.0) * time.delta_seconds()
        } else {
            Vec3::ZERO
        };

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
            if move_vector.length() > 0.001 {
                controller.translation = Some(move_vector);
            }
        } else {
            let forward = -movable_tranform.forward();
            move_vector += forward * movable_data.speed * time.delta_seconds();
            controller.translation = Some(move_vector);
        }

        if let Some(player_history) = player_history.as_mut() {
            let speed = controller_output.effective_translation.length();
            player_history.log(speed, time.delta_seconds());
        }
    }
}

fn move_movables_enemy(
    mut movables: Query<(&mut Transform, &mut Movable), With<EnemyTag>>,
    time: Res<Time>,
) {
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
            if !animator.is_playing_clip(&movable_animation.idle_animations[0]) {
                // hack to fix feet position
                animator
                    .start_with_transition(
                        movable_animation.idle_animations[0].clone_weak(),
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

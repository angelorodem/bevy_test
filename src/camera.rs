use std::time::Duration;

use bevy::{
    core_pipeline::{
        bloom::BloomSettings, experimental::taa::TemporalAntiAliasBundle, tonemapping::Tonemapping,
    },
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::*,
};

use crate::player::PlayerTag;

const CAMERA_DISTANCE: f32 = 20.0;
const CAMERA_HEIGHT: f32 = 5.0;
const CAMERA_OFFSET: Vec3 = Vec3::new(CAMERA_DISTANCE, CAMERA_HEIGHT, CAMERA_DISTANCE);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(CAMERA_DISTANCE, CAMERA_DISTANCE, CAMERA_DISTANCE),
            // tonemapping: Tonemapping::TonyMcMapface,
            ..Default::default()
        })
        .insert(BloomSettings::NATURAL)
        .insert(ScreenSpaceAmbientOcclusionBundle::default())
        .insert(TemporalAntiAliasBundle::default())
        .insert(ScreenSpaceAmbientOcclusionSettings {
            quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
        });
}

fn player_camera(
    player: Query<&Transform, (With<PlayerTag>, Without<Camera3d>)>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<PlayerTag>)>,
    time: Res<Time>,
) {
    for player_transform in player.iter() {
        for mut camera_transform in camera.iter_mut() {
            //Look at the player
            camera_transform.look_at(player_transform.translation, Vec3::Y);

            //Move the camera to the right distance
            camera_transform.translation = camera_transform.translation.lerp(
                player_transform.translation + CAMERA_OFFSET,
                time.delta_seconds(),
            );
        }
    }
}

mod asset_loader;
mod camera;
mod character;
mod enemy;
mod movable;
mod player;

use std::time::Duration;

use asset_loader::AssetLoaderPlugin;
use bevy::{
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

//https://github.com/djeedai/bevy_tweening
use bevy_tweening::*;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use movable::MovablePlugin;
use player::PlayerPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(TweeningPlugin)
        .add_plugins(TemporalAntiAliasPlugin)
        // User Plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        // .add_plugins(EnemyPlugin)
        .add_plugins(MovablePlugin)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.8))
        .insert(Restitution::coefficient(0.2))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.8,
                    subdivisions: 5,
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(StandardMaterial {
                emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
                ..default()
            }),
            transform: Transform::from_xyz(4.0, 5.0, 0.0),
            ..Default::default()
        });
}

mod asset_loader;
mod camera;
mod character;
mod debug;
mod enemy;
mod environment;
mod movable;
mod player;
mod states;

use asset_loader::AssetLoaderPlugin;
use bevy::{core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*};
use bevy_rapier3d::prelude::*;

//https://github.com/djeedai/bevy_tweening
use camera::CameraPlugin;
use debug::GuiDebugPlugin;
use enemy::EnemyPlugin;
use environment::EnvironmentPlugin;
use movable::MovablePlugin;
use player::PlayerPlugin;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // Debug
        .add_plugins(GuiDebugPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(TemporalAntiAliasPlugin)
        // User Plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
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
        .insert(ActiveEvents::COLLISION_EVENTS)
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
            transform: Transform::from_xyz(4.0, 1.8, 0.0),
            ..Default::default()
        })
        .insert(SolverGroups::new(Group::GROUP_32, Group::GROUP_32))
        .insert(CollisionGroups::new(Group::GROUP_32, Group::GROUP_32));
}

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
        .add_plugins(TweeningPlugin)
        // Diagnostics
        .add_plugins((
            // Adds a system that prints diagnostics to the console
            LogDiagnosticsPlugin {
                wait_duration: Duration::from_secs(10),
                ..Default::default()
            },
            FrameTimeDiagnosticsPlugin,
            // Any plugin can register diagnostics. Uncomment this to add an entity count diagnostics:
            bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
            // Uncomment this to add system info diagnostics:
            // bevy::diagnostic::SystemInformationDiagnosticsPlugin::default(),
        ))
        .add_plugins(TemporalAntiAliasPlugin)
        // User Plugins
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(MovablePlugin)
        .run();
}

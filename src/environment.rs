use bevy::prelude::*;
use bevy_rapier3d::{dynamics::RigidBody, geometry::Collider};

use crate::asset_loader::EnvironmentAssets;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_environment);
    }
}

fn load_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enviroment_assets: Res<EnvironmentAssets>,
) {
    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        sectors: 128,
        stacks: 64,
        ..default()
    }));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(1.0, 4.0, 0.0),
        point_light: PointLight {
            intensity: 3000.0,
            radius: 2.0,
            color: Color::rgb(0.2, 0.2, 1.0),
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                emissive: Color::rgb_linear(0.2, 0.2, 10.0),
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 4.0, 0.0).with_scale(Vec3::splat(0.8)),
            ..default()
        })
        .with_children(|children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 15000.0,
                    radius: 0.8,
                    shadows_enabled: true,
                    color: Color::rgb(0.2, 0.2, 1.0),
                    ..default()
                },
                ..default()
            });
        });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(100.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(enviroment_assets.chess_texture.clone()),
                alpha_mode: AlphaMode::Opaque,
                perceptual_roughness: 0.08,
                ..default()
            }),
            ..default()
        })
        .insert((Collider::cuboid(50.0, 0.1, 50.0),))
        .insert(RigidBody::Fixed);

    // test block
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(4.0, 4.0, 4.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(enviroment_assets.chess_texture.clone()),
                alpha_mode: AlphaMode::Opaque,
                ..default()
            }),
            transform: Transform::from_xyz(-4.0, 2.0, -4.0),
            ..default()
        })
        .insert((Collider::cuboid(2.0, 2.0, 2.0), RigidBody::Fixed));

    // test block2
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(4.0, 1.0, 4.0).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(enviroment_assets.chess_texture.clone()),
                alpha_mode: AlphaMode::Opaque,
                ..default()
            }),
            transform: Transform::from_xyz(1.0, 0.5, -4.0),
            ..default()
        })
        .insert((Collider::cuboid(2.0, 0.5, 2.0), RigidBody::Fixed));
}

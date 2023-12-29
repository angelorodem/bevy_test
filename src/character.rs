use bevy::prelude::*;

#[derive(Component)]
pub struct NameComponent(pub String);

#[derive(Component)]
pub struct HealthComponent(pub f32);

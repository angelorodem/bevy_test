use bevy::{prelude::*, transform::commands, ecs::query};

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Jhon".to_string())));
    commands.spawn((Person, Name("aaa".to_string())));
    commands.spawn((Person, Name("Jhoeeen".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

//https://bevyengine.org/learn/book/getting-started/plugins/

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, add_people)
    .add_systems(Update, (hello_world, greet_people))
    .run();
}
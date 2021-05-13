use bevy::prelude::*;

struct Name(&'static str);

struct Helath(u16);

struct Monster;

struct Hunter;

fn setup(mut commands: Commands) {
    commands.spawn_bundle((Hunter, Name("Alice"), Helath(1000)));
    commands.spawn_bundle((Hunter, Name("Bob"), Helath(1000)));
    commands.spawn_bundle((Hunter, Name("Charlie"), Helath(1000)));
    commands.spawn_bundle((Monster, Name("Iceborne"), Helath(5000)));
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Hunting Crustaceans".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run()
}

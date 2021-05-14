use bevy::prelude::*;
use rand::{thread_rng, Rng};

struct Name(&'static str);

struct Position {
    x: i32,
    y: i32,
}

struct Speed(u16);

struct Damage(u16);

struct Helath(u16);

struct Score(u16);

struct Monster;

struct Hunter;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle((Hunter, Name("Alice"), Helath(1000)));
    commands.spawn_bundle((Hunter, Name("Bob"), Helath(1000)));
    commands.spawn_bundle((Hunter, Name("Charlie"), Helath(1000)));
    commands.spawn_bundle((Monster, Name("Iceborne"), Helath(5000)));

    let players = (0..=3).map(|_| {
        let pos = Position { x: 0, y: 0 };

        (Hunter, pos, Speed(5), Helath(1000), Damage(50), Score(0))
    });

    commands.spawn_batch(players);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_xyz(0.0, 50.0, 1.0),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .insert(Monster);

    let crusta = asset_server.load("res/icon.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(crusta.into()),
        transform: Transform {
            translation: Vec3::new(-50.0, -50.0, 0.0),
            scale: Vec3::new(0.3, 0.3, 0.3),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Hunter);
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

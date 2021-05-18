use bevy::prelude::*;
use bevy::{core::FixedTimestep, math::Vec3Swizzles};
use rand::{thread_rng, Rng};

const TIME_STEP: f32 = 1.0 / 2.0;

struct MyEvent {
    pub message: String,
}

struct EventTriggerState {
    event_timer: Timer,
}

impl Default for EventTriggerState {
    fn default() -> Self {
        EventTriggerState {
            event_timer: Timer::from_seconds(1.0, true),
        }
    }
}

struct Name(&'static str);

struct Position {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Speed(Vec2);

struct Damage(u16);

struct Helath(u16);

struct Score(u16);

struct AI(f32);

struct Monster;

struct Hunter;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    /*
    commands.spawn_bundle((Hunter, Name("Alice"), Helath(1000)));
    commands.spawn_bundle((Hunter, Name("Bob"), Helath(1000)));
    commands.spawn_bundle((Hunter, Name("Charlie"), Helath(1000)));
    //commands.spawn_bundle((Monster, Name("Iceborne"), Helath(5000)));

    let players = (0..=3).map(|_| {
        let pos = Position { x: 0, y: 0 };

        (Hunter, pos, Speed(5), Helath(1000), Damage(50), Score(0))
    });

    commands.spawn_batch(players);
    */

    let crusta = asset_server.load("res/icon.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(crusta.into()),
            transform: Transform {
                translation: Vec3::new(20.0, 0.0, 0.0),
                scale: Vec3::new(0.3, 0.3, 0.3),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Speed(Vec2::new(1.0, 1.0)))
        .insert(Monster);

    let crusta2 = asset_server.load("res/ferris-ninja-min.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(crusta2.into()),
            transform: Transform {
                translation: Vec3::new(-450.0, -250.0, 0.0),
                scale: Vec3::new(0.1, 0.1, 0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Speed(Vec2::new(1.0, 1.0)))
        .insert(AI(1.0))
        .insert(Hunter);

    let crusta = asset_server.load("res/ferris-viking-min.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(crusta.into()),
            transform: Transform {
                translation: Vec3::new(450.0, -250.0, 0.0),
                scale: Vec3::new(0.1, 0.1, 0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Speed(Vec2::new(1.0, 1.0)))
        .insert(AI(0.1))
        .insert(Hunter);
}

//fn movement_system(time: Res<Time>, mut query: Query<(&Monster, &Speed, &mut Transform)>) {
fn movement_system(time: Res<Time>, mut query: Query<(&Speed, &mut Transform)>) {
    for (speed, mut trans) in query.iter_mut() {
        trans.translation += speed.0.extend(0.0) * time.delta_seconds();
    }
}

fn acc_movement_system(mut monster_query: Query<(&Monster, &mut Speed, &Transform)>) {
    if let Ok((m, mut s, t)) = monster_query.single_mut() {
        let mut rng = rand::thread_rng();

        s.0 = 100.0 * Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
    }
}

fn ai_system(
    mut monster_query: Query<(&Monster, &mut Speed, &Transform), Without<Hunter>>,
    mut hunter_query: Query<(&Hunter, &mut Speed, &Transform, &AI), Without<Monster>>,
) {
    if let Ok((_m, mut m_speed, m_trans)) = monster_query.single_mut() {
        println!("In AI we see monster in: {}", m_trans.translation);

        m_speed.0 = 100.0 * Vec2::new(thread_rng().gen_range(-1.0..1.0), thread_rng().gen_range(-1.0..1.0)).normalize();

        for (_h, mut speed, h_trans, ai) in hunter_query.iter_mut() {
            println!("Hunter is in {}", h_trans.translation);

            let mut v: Vec3 = (m_trans.translation - h_trans.translation).normalize();

            v.x += thread_rng().gen_range(-ai.0..ai.0);
            v.y += thread_rng().gen_range(-ai.0..ai.0);

            println!("AI v: {}", v);
            speed.0 = 80.0 * v.xy().normalize();
        }
    }
}
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Hunting Crustaceans".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(movement_system.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                //.with_system(acc_movement_system.system())
                .with_system(ai_system.system()),
        )
        .run()
}

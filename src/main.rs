use bevy::prelude::*;
use bevy::{core::FixedTimestep, math::Vec3Swizzles};
use bevy_rapier2d::na::{Complex, Unit, Vector2};
use bevy_rapier2d::physics::{EventQueue, RigidBodyHandleComponent};
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::rapier::{
    dynamics::{RigidBodyBuilder, RigidBodySet},
    geometry::{ColliderBuilder, ColliderSet, ContactEvent},
};
use std::{borrow::Borrow, collections::HashMap};

use rand::{thread_rng, Rng};
use std::{thread, time};

mod components;
mod explosion;

use components::*;
use explosion::*;

const TIME_STEP: f32 = 1.0 / 1.0;

#[derive(Debug)]
struct Name(&'static str);

#[derive(Debug)]
struct Speed(Vec2);

#[derive(Debug)]
struct Damage(u16);

#[derive(Debug)]
struct Helath(u16);

struct Score(u16);

struct AI(f32);

struct Monster;

struct Hunter;

#[derive(Debug)]
struct GameState;

fn spawn_hunter(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut hunters = HashMap::new();
    hunters.insert("Alice", "res/ferris-miner-min.png");
    hunters.insert("Bob", "res/ferris-ninja-min.png");
    hunters.insert("Charlie", "res/ferris-viking-min.png");

    for (name, logo) in hunters {
        let crusta = asset_server.load(logo);

        let mut h = commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(1.0 / 9.0),
                ..Default::default()
            },
            material: materials.add(crusta.into()),
            ..Default::default()
        });

        let body = RigidBodyBuilder::new_dynamic()
            .translation(
                rand::thread_rng().gen_range(-400.0..400.0),
                rand::thread_rng().gen_range(-300.0..300.0),
            )
            .user_data(h.id().to_bits() as u128);

        let collider = ColliderBuilder::ball(40.0);

        h.insert_bundle((body, collider));
        h.insert_bundle((
            Hunter,
            Name(name),
            Speed(Vec2::new(0.0, 0.0)),
            AI(rand::thread_rng().gen_range(0.0..1.0)),
            Damage(20),
        ));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("res/pexels-francesco-ungaro-998641-min.png");
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            //scale: Vec3::splat(0.1),
            ..Default::default()
        },
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });

    let ten_millis = time::Duration::from_millis(1000);

    //thread::sleep(ten_millis);

    let crusta = asset_server.load("res/icon.png");
    let mut m = commands.spawn_bundle(SpriteBundle {
        material: materials.add(crusta.into()),
        transform: Transform {
            scale: Vec3::new(0.3, 0.3, 0.3),
            ..Default::default()
        },
        ..Default::default()
    });

    let body = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 0.0)
        .lock_rotations()
        .user_data(m.id().to_bits() as u128);

    let collider = ColliderBuilder::ball(40.0);
    //.insert(RigidBodyBuilder::new_dynamic())
    //.insert(ColliderBuilder::cuboid(1.0, 1.0))
    m.insert_bundle((body, collider))
        .insert(Speed(Vec2::new(0.0, 0.0)))
        .insert(Helath(100))
        .insert(Name("Bevy"))
        .insert(Monster);

    spawn_hunter(commands, materials, asset_server);
}

//fn movement_system(time: Res<Time>, mut query: Query<(&Monster, &Speed, &mut Transform)>) {
fn movement_system(time: Res<Time>, mut query: Query<(&Speed, &mut Transform)>) {
    for (speed, mut trans) in query.iter_mut() {
        trans.translation += speed.0.extend(0.0) * time.delta_seconds();
    }
    //println!("---------------------------")
}

fn game_ending(mut commands: Commands, mut event_reader: EventReader<GameState>) {
    for event in event_reader.iter() {
        println!("{:?}", event);

        let mut m = commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(0.3, 0.3, 0.3),
                ..Default::default()
            },
            ..Default::default()
        });

        let body = RigidBodyBuilder::new_dynamic()
            .translation(0.0, 0.0)
            .lock_rotations()
            .user_data(m.id().to_bits() as u128);

        let collider = ColliderBuilder::ball(10.0);
        //.insert(RigidBodyBuilder::new_dynamic())
        //.insert(ColliderBuilder::cuboid(1.0, 1.0))
        m.insert_bundle((body, collider))
            .insert(Speed(Vec2::new(0.0, 0.0)))
            .insert(Name("DeadBevy"))
            .insert(Monster);
    }
}

fn position_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<(&Speed, &RigidBodyHandleComponent)>,
) {
    for (player, rigid_body_component) in query.iter() {
        let m = Vector2::new(player.0.x, player.0.y);
        if let Some(rb) = bodies.get_mut(rigid_body_component.handle()) {
            if rb.angvel() != 0.0 {
            } else {
                rb.set_linvel(m * time.delta_seconds() * 100.0, true);
            }

            let mut pos = rb.position().clone();

            if rb.position().translation.vector.x > 600.0
                || rb.position().translation.vector.x < -600.0
                || rb.position().translation.vector.y > 300.0
                || rb.position().translation.vector.y < -300.0
            {
                //let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic).rotation(0.0);
                rb.set_angvel(0.0, false);
                //pos.rotation = rigid_body.position().rotation;
                //rb.set_position(pos, false);
                pos.rotation = Unit::new_normalize(Complex::new(0.1, 0.0));
                //println!("{}", rb.position().rotation);
                rb.set_position(pos, false);
            }
        }
    }

    /*
    for (speed, body_handle) in &mut query.iter() {
        let body = bodies.get_mut(body_handle.handle()).unwrap();
        let mut x = body.position().translation.vector.x;
        let mut y = body.position().translation.vector.y;

        //println!("{} -- {} with {}", x, y, speed.0);

        let mut new_position = body.position().clone();
        new_position.translation.vector.x += speed.0.x * time.delta_seconds();
        new_position.translation.vector.y += speed.0.y * time.delta_seconds();
        body.set_position(new_position, false);

    }
    */
}

fn ai_system(
    mut monster_query: Query<(&Monster, &mut Speed, &Transform), Without<Hunter>>,
    mut hunter_query: Query<(&Hunter, &mut Speed, &Transform, &AI), Without<Monster>>,
) {
    if let Ok((_m, mut m_speed, m_trans)) = monster_query.single_mut() {
        println!("In AI we see monster in: {}", m_trans.translation);

        m_speed.0 = 20.0
            * Vec2::new(
                thread_rng().gen_range(-1.0..1.0),
                thread_rng().gen_range(-1.0..1.0),
            )
            .normalize();

        for (_h, mut speed, h_trans, ai) in hunter_query.iter_mut() {
            //println!("Hunter is in {}", h_trans.translation);

            let mut v: Vec3 = (m_trans.translation - h_trans.translation).normalize();

            v.x += thread_rng().gen_range(-ai.0..ai.0);
            v.y += thread_rng().gen_range(-ai.0..ai.0);

            //println!("AI v: {}", v);
            speed.0 = 40.0 * v.xy().normalize();
        }
    }
}

fn check_collision_events(
    mut commands: Commands,
    events: Res<EventQueue>,
    bodies: ResMut<RigidBodySet>,
    colliders: ResMut<ColliderSet>,
    asset_server: Res<AssetServer>,
    mut explosion_spawn_events: EventWriter<ExplosionSpawnEvent>,
    mut game_ending_events: EventWriter<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Name, &mut Helath), Without<Hunter>>,
    hquery: Query<(&Name, &Damage), Without<Monster>>,
) {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(c0, c1) => {
                let collider0 = colliders.get(c0).unwrap();
                let collider1 = colliders.get(c1).unwrap();

                let b0 = bodies.get(collider0.parent()).unwrap();
                let b1 = bodies.get(collider1.parent()).unwrap();
                //let b1 = bodies.get_mut(collider1.parent()).unwrap();
                //let b0 = bodies.get(collider0.parent()).unwrap();

                let e0 = Entity::from_bits(b0.user_data as u64);
                let e1 = Entity::from_bits(b1.user_data as u64);

                //println!("{:?}", query.get_component::<Name>(e0).unwrap());
                //println!("{:?}", query.get_component::<Name>(e1).unwrap());

                //let a = commands.entity(e1).get_mut::<Helath>().unwrap();
                if let Ok((mn, mut h)) = query.get_mut(e0) {
                    println!("{:?} now at {}", mn, h.0);
                    if let Ok((hn, d)) = hquery.get(e1) {
                        println!("{} hit monst with {}", hn.0, d.0);
                        h.0 -= d.0;
                    }
                    if h.0 == 0 {
                        commands.entity(e0).despawn();

                        commands.spawn_bundle(SpriteBundle {
                            material: materials
                                .add(asset_server.load("res/game_over_transparent.png").into()),
                            ..Default::default()
                        });

                        game_ending_events.send(GameState {});

                        explosion_spawn_events.send(ExplosionSpawnEvent {
                            kind: ExplosionKind::ShipDead,
                            x: b0.position().translation.x,
                            y: b0.position().translation.y,
                        });
                    }
                }

                /*
                if let Ok(a) = query.get_component::<Name>(e0) {
                    println!("{:?} now at {}", a, h.0);
                    if let Ok(n) = hquery.get_component::<Name>(e1) {
                        let d = hquery.get_component::<Damage>(e1).unwrap();
                        println!("{} hit monst with {}", n.0, d.0);
                        h.0 -= d.0;
                    }
                }
                */

                println!("-----------------------------------------------")
                //commands.entity(e1).despawn();
                //commands.entity(e2).despawn();

                /*
                //b0.apply_force(b1.position().translation, false);
                //b1.apply_force(b0.position().translation, false);
                b1.apply_force(Vector2::new(-1000.0, -1000.0), false);

                //println!("{:?}", b0.position().translation);
                println!("{:?}", b1.position().translation);
                */
            }
            ContactEvent::Stopped(_, _) => {}
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .insert_resource(WindowDescriptor {
            title: "Hunting Crustaceans".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vector2::zeros(),
            time_dependent_number_of_timesteps: true, //physic run at fixed 60Hz
            ..Default::default()
        })
        .add_event::<ExplosionSpawnEvent>()
        .add_event::<GameState>()
        .add_startup_system(setup.system())
        //.add_system(movement_system.system())
        .add_system(position_system.system())
        .add_system(check_collision_events.system())
        .add_system(handle_explosion.system())
        .add_system(spawn_explosion_event.system())
        .add_system(game_ending.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                //.with_system(acc_movement_system.system())
                .with_system(ai_system.system()),
        )
        .run()
}

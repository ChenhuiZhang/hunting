use super::components::*;
use bevy::prelude::*;

pub struct Explosion {
    timer: Timer,
    start_scale: f32,
    end_scale: f32,
}

pub fn spawn_explosion_event(
    mut commands: Commands,
    mut event_reader: EventReader<ExplosionSpawnEvent>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.iter() {
        let (texture_name, _sound_name, start_scale, end_scale, duration) = match event.kind {
            ExplosionKind::ShipDead => (
                "res/explosion01.png",
                "res/Explosion_ship.mp3",
                0.1 / 15.0,
                3.0 / 15.0,
                2.5,
            ),
            ExplosionKind::ShipContact => {
                ("flash00.png", "Explosion.mp3", 0.05 / 15.0, 0.1 / 15.0, 0.5)
            }
            ExplosionKind::LaserOnAsteroid => {
                ("flash00.png", "Explosion.mp3", 0.1 / 15.0, 0.15 / 15.0, 0.5)
            }
        };

        let texture_handle = asset_server.load(texture_name);
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(event.x, event.y, 1.0),
                    scale: Vec3::splat(start_scale),
                    ..Default::default()
                },
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .insert(Explosion {
                timer: Timer::from_seconds(duration, false),
                start_scale,
                end_scale,
            });
        //let sound = asset_server.load(sound_name);
        //audio.play(sound);
    }
}

pub fn handle_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Explosion)>,
) {
    let elapsed = time.delta();
    for (entity, mut transform, mut explosion) in query.iter_mut() {
        explosion.timer.tick(elapsed);
        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.scale = Vec3::splat(
                explosion.start_scale
                    + (explosion.end_scale - explosion.start_scale)
                        * (explosion.timer.elapsed_secs()
                            / explosion.timer.duration().as_secs_f32()),
            );
        }
    }
}

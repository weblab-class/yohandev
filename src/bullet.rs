use hecs::{ World, EntityBuilder };

use crate::{
    math::Vec2,
    physics::{ Collider, KinematicBody },
    transform::Transform,
    network::Packet,
    platform::{ Socket, Time },
    render::{ Sprite, Costume },
};

// TODO: this is a lazy workaround for now, but a system like this could be
// nice for general network architecture.
/// Component that marks this entity as having payload to send.
struct Payload(Option<Packet>);

/// Component for entity that should life for `.0` seconds
struct BulletLifetime(f32);

/// Create a bullet locally
pub fn prefab(origin: Vec2<f32>, velocity: Vec2<f32>) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Sprite::new(Costume::Bullet {
            position: origin,
        }),
        Collider::circle(3.0),
        KinematicBody { velocity },
        Transform {
            translation: origin,
            ..Default::default()
        },
        BulletLifetime(2.0),
    ));
    // Replicate on the network.
    if cfg!(server) {
        builder.add(Payload(Some(Packet::ProjectileSpawn {
            origin,
            velocity
        })));
    }
    builder
}

/// System that creates bullets on the network
pub fn network_instantiate(world: &mut World, socket: &Socket) {
    // Server replicates bullets
    if cfg!(server) {
        for (_, Payload(data)) in world.query_mut::<&mut Payload>() {
            // Take the packet so it doesn't send twice
            if let Some(packet) = data.take() {
                socket.broadcast(&packet);
            }
        }
    }
    // Client spawns whatever it's told to
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::ProjectileSpawn { origin, velocity } = packet else {
                continue;
            };
            world.spawn(prefab(*origin, *velocity).build());
        }
    }
}

/// System that automatically despawns stale bullets
pub fn despawn_bullets(world: &mut World, time: &Time) {
    let mut kill = Vec::new();
    
    for (e, BulletLifetime(t)) in world.query_mut::<&mut BulletLifetime>() {
        *t -= time.dt();
        if *t <= 0.0 {
            kill.push(e);
        }
    }
    for e in kill {
        world.despawn(e).unwrap();
    }
}
use hecs::{ World, EntityBuilder, With };

use crate::{
    math::Vec2,
    physics::{ Collider, KinematicBody, Collisions, FixedBody },
    transform::Transform,
    network::Packet,
    platform::{ Socket, Time },
    render::{ Sprite, Costume }, health::{Damage, Health},
};

// TODO: this is a lazy workaround for now, but a system like this could be
// nice for general network architecture.
/// Component that marks this entity as having payload to send.
struct Payload(Option<Packet>);

/// Component for entity that should life for `.0` seconds
struct BulletLifetime(f32);

/// Create a bullet locally
pub fn prefab(origin: Vec2<f32>, velocity: Vec2<f32>, ttl: f32) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Sprite::new(Costume::Bullet {
            position: origin,
        }),
        Collider::circle(3.0),
        Collisions::default(),
        KinematicBody { velocity },
        Transform {
            translation: origin,
            ..Default::default()
        },
        BulletLifetime(ttl),
    ));
    // Replicate on the network.
    if cfg!(server) {
        builder.add(Payload(Some(Packet::ProjectileSpawn {
            origin,
            velocity,
            ttl
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
            let Packet::ProjectileSpawn { origin, velocity, ttl } = packet else {
                continue;
            };
            world.spawn(prefab(*origin, *velocity, *ttl).build());
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

/// System that deals damage to entities with [Health]
pub fn impact_and_damage(world: &mut World, socket: &Socket) {
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::EntityHealth(e, hitpoints) = packet else {
                continue;
            };
            if let Ok(mut health) = world.get::<&mut Health>(*e) {
                health.now = *hitpoints;
            }
        }
    }
    let mut destroy = Vec::new();
    // Query bullets
    for (e1, (damage, collisions)) in &mut world.query::<(&Damage, &Collisions)>() {
        for &e2 in &collisions.0 {
            if let Some(e3) = damage.exclude {
                if e2 == e3 {
                    continue;
                }
            }
            // Destroy
            if damage.destroy && destroy.last() != Some(&e1) {
                // No bullet/bullet collisions
                if matches!(world.satisfies::<&BulletLifetime>(e2), Ok(true)) {
                    continue;
                }
                destroy.push(e1);
            }
            // Health
            let Ok(mut health) = world.get::<&mut Health>(e2) else {
                continue;
            };
            if cfg!(server) {
                // Inflict damage
                health.now = (health.now - damage.amount).max(0.0);
                // Tell clients
                socket.broadcast(&Packet::EntityHealth(e2, health.now));
            }
        }
    }
    // Client can't know when bullets hit, so small visual hack is
    // to just stop them when something static is hit
    if cfg!(client) {
        for (e, collisions) in &mut world.query::<With<&Collisions, &BulletLifetime>>() {
            for &e2 in &collisions.0 {
                if matches!(world.satisfies::<&FixedBody>(e2), Ok(true)) {
                    destroy.push(e);
                    break;
                }
            }
        }
    }
    for e in destroy {
        world.despawn(e).unwrap();
    }
}
use hecs::{ Entity, EntityBuilder, World };

use crate::{
    render::{ Sprite, Costume },
    transform::{ Transform, Parent, LocalPosition },
    math::vec2, bullet::TimeToLive, platform::{Time, Socket}, player::instantiate_spawn_indicator, network::Packet,
};

/// Component for an entity's health
pub struct Health {
    /// Current numper of hitpoints
    pub now: f32,
    /// Hitpoints when health resets
    pub max: f32,
}

/// Component for entities that deal damage when come in
/// contact with a [Health] entity's [Collisions]
pub struct Damage {
    /// Hitpoints subtracted from [Health]
    pub amount: f32,
    /// Entity to exclude from taking damage, ie. the shooter of this bullet
    pub exclude: Option<Entity>,
    /// Whether this entity should be destroyed after?
    pub destroy: bool,
}

/// Prefab for the healthbar
pub fn gui_prefab(owner: Entity) -> EntityBuilder {
    let mut builder = EntityBuilder::new();

    builder.add_bundle((
        Sprite::new(Costume::HealthBar {
            position: Default::default(),
            percentage: 1.0,
        }),
        Transform::default(),
        Parent(owner),
        LocalPosition(vec2!(0.0, 40.0)),
    ));
    builder
}

struct RespawnTimer {
    player: Entity,
    left: f32,
}

/// System that respawns players on death
pub fn respawn_players(world: &mut World, socket: &Socket, time: &Time) {
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::PlayerRespawn(e, pos) = packet else {
                continue;
            };
            if let Ok(mut transform) = world.get::<&mut Transform>(*e) {
                transform.translation = *pos;
            }
            instantiate_spawn_indicator(world, *e);
        }
        return;
    }
    // Kill players and remove them from the map
    let mut kill = Vec::new();
    for (e, health) in world.query_mut::<&mut Health>() {
        if health.now <= 0.0 {
            // Reset
            health.now = health.max;
            socket.broadcast(&Packet::EntityHealth(e, health.now));
            kill.push(e);
        }
    }
    for e in kill {
        if let Ok(mut transform) = world.get::<&mut Transform>(e) {
            transform.translation = vec2!(-2000.0, 5000.0);
        }
        world.spawn((
            RespawnTimer {
                left: 2.0,
                player: e,
            },
        ));
    }
    // Respawn players and put them back on the map
    let mut rm = Vec::new();
    for (e, timer) in &mut world.query::<&mut RespawnTimer>() {
        timer.left -= time.dt() / time.scale;

        if timer.left > 0.0 {
            continue;
        }
        let pos = vec2!(100.0, 500.0);
        if let Ok(mut transform) = world.get::<&mut Transform>(timer.player) {
            transform.translation = pos;
        }
        socket.broadcast(&Packet::PlayerRespawn(timer.player, pos));
        rm.push(e);
    }
    for e in rm {
        world.despawn(e).unwrap();
    }
}
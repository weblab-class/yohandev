use hecs::{World, Entity, EntityBuilder};

use crate::{
    physics::Collisions,
    render::{ Sprite, Costume },
    transform::{ Transform, Parent, LocalPosition },
    math::vec2, platform::Socket, network::Packet,
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

/// System that deals damage to entities with [Health]
pub fn deal_damage(world: &mut World, socket: &Socket) {
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
    for (e1, (health, collisions)) in &mut world.query::<(&mut Health, &Collisions)>() {
        for e2 in &collisions.0 {
            let Ok(damage) = world.get::<&Damage>(*e2) else {
                continue;
            };
            if let Some(e3) = damage.exclude {
                if e1 == e3 {
                    continue;
                }
            }
            if cfg!(server) {
                // Inflict damage
                health.now = (health.now - damage.amount).max(0.0);
                // Tell clients
                socket.broadcast(&Packet::EntityHealth(e1, health.now));
            }
            // Destroy bullet
            if damage.destroy {
                destroy.push(*e2);
            }
        }
    }
    for e in destroy {
        world.despawn(e).unwrap();
    }
}
use hecs::{ World, Entity, With };

use crate::{
    ability::{ Ability, Cooldown },
    platform::{Time, Socket, Connection},
    transform::Transform,
    math::vec2,
    render::{ Sprite, Costume }, bullet::TimeToLive, network::Packet, health::Health,
};

/// Component that marks this entity as the heal ability
struct Heal;

pub fn instantiate(world: &mut World, owner: Entity, binding: usize) -> Entity {
    world.spawn((
        Ability {
            owner,
            binding,
            active: false,
        },
        Heal,
        Cooldown::default(),
    ))
}

/// System that controls the heal ability
pub fn heal_controller(world: &mut World, time: &Time, socket: &Socket) {
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::EffectSpawn(costume) = packet else {
                continue;
            };
            if !matches!(costume, Costume::Heal { .. }) {
                continue;
            }
            // Add sprite
            world.spawn((
                Sprite::new(costume.clone()),
                TimeToLive::Frames(100)
            ));
        }
        return;
    }
    type Query<'a> = With<(&'a Ability, &'a mut Cooldown), &'a Heal>;

    for (_, (ability, cooldown)) in &mut world.query::<Query>() {
        // Cooldown
        cooldown.0 -= time.dt();
        // Trigger
        if ability.active && cooldown.0 <= 0.0 {
            if let Ok(transform) = world.get::<&Transform>(ability.owner) {
                // Sprite
                socket.broadcast(&Packet::EffectSpawn(Costume::Heal {
                    position: transform.translation - vec2!(0.0, 30.0),
                }));
            }
            if let Ok(mut health) = world.get::<&mut Health>(ability.owner) {
                health.now = (health.now + 20.0).min(health.max);
                socket.broadcast(&Packet::EntityHealth(ability.owner, health.now));
            }
            *cooldown = Cooldown(5.0);
            if let Ok(id) = world.get::<&Connection>(ability.owner) {
                socket.send(*id, &Packet::CooldownStart {
                    binding: ability.binding,
                    duration: cooldown.0,
                })
            }
        }
    }
}
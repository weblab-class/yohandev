use hecs::{ World, Entity, With };

use crate::{
    ability::{ Ability, Cooldown },
    platform::{Time, Socket, Connection},
    transform::Transform,
    physics::KinematicBody,
    render::{ Sprite, Costume }, bullet::TimeToLive, network::Packet,
};

/// Component that marks this entity as the push ability
struct Push;

pub fn instantiate(world: &mut World, owner: Entity, binding: usize) -> Entity {
    world.spawn((
        Ability {
            owner,
            binding,
            active: false,
        },
        Push,
        Cooldown::default(),
    ))
}

/// System that controls the almighty push
pub fn push_controller(world: &mut World, time: &Time, socket: &Socket) {
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::EffectSpawn(costume) = packet else {
                continue;
            };
            if !matches!(costume, Costume::Push { .. }) {
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
    /// Queries all weapon holders
    type Query<'a> = With<(&'a Ability, &'a mut Cooldown), &'a Push>;

    let mut pushes = Vec::new();
    for (_, (ability, cooldown)) in &mut world.query::<Query>() {
        // Cooldown
        cooldown.0 -= time.dt();
        // Trigger
        if ability.active && cooldown.0 <= 0.0 {
            if let Ok(transform) = world.get::<&Transform>(ability.owner) {
                pushes.push(transform.translation);
            }
            *cooldown = Cooldown(15.0);
            if let Ok(id) = world.get::<&Connection>(ability.owner) {
                socket.send(*id, &Packet::CooldownStart {
                    binding: ability.binding,
                    duration: cooldown.0,
                })
            }
        }
    }
    for origin in pushes {
        // Sprite
        socket.broadcast(&Packet::EffectSpawn(Costume::Push { position: origin }));
        // Push everything
        for (_, (t, kb)) in world.query_mut::<(&Transform, &mut KinematicBody)>() {
            if let Some(delta) = (t.translation - origin).try_normalize(0.01) {
                kb.velocity = 2000.0 * delta;
            }
        }
    }
}
use hecs::{World, Entity};

use crate::{
    math::Vec2,
    platform::{Time, Connection, Socket},
    ability::Ability,
    transform::Transform,
    input::Input, network::Packet,
};

/// Component for a generic gun's stats.
pub struct Gun {
    /// Gun's cooldown after each shot
    pub cooldown: Cooldown,
    /// Function that instantiates bullets 
    pub shoot: fn(world: &mut World, owner: Entity, origin: Vec2<f32>, velocity: Vec2<f32>),
}

/// Component for current cooldown time.
#[derive(Debug, Default, Clone, Copy)]
pub struct Cooldown(pub f32);

/// System that does the generic gun functionality
pub fn gun_controller(world: &mut World, socket: &Socket, time: &Time) {
    if cfg!(client) {
        return;
    }
    /// Queries all weapon holders
    type Query<'a> = (
        &'a Ability,            // Needed to test if active or not
        &'a Gun,                // Guns properties
        &'a mut Cooldown,       // Test and reset cooldown
        &'a mut Transform,      // Origin of bullets
    );
    let mut shots = Vec::new();
    for (_, (ability, gun, cooldown, transform)) in &mut world.query::<Query>() {
        // User input
        let Ok(input) = world.get::<&Input>(ability.owner) else {
            continue;
        };
        // Cooldown
        cooldown.0 -= time.dt();
        // Shooting
        if ability.active && cooldown.0 <= 0.0 && input.fire() {
            shots.push((gun.shoot, ability.owner, transform.translation, input.look_axis()));
            *cooldown = gun.cooldown;
            // "some" impatient threshold
            if cooldown.0 > 0.7 {
                if let Ok(id) = world.get::<&Connection>(ability.owner) {
                    socket.send(*id, &Packet::CooldownStart {
                        binding: ability.binding,
                        duration: cooldown.0,
                    })
                }
            }
        }
    }
    for (shoot, e, o, v) in shots {
        (shoot)(world, e, o, v);
    }
}
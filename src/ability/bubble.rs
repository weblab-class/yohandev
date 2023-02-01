use hecs::{ Entity, World };

use crate::{
    ability::Ability,
    render::{ Sprite, Costume },
    transform::{ Transform, LocalPosition },
    input::FollowLookDirection,
    physics::Collider,
    math::{ Rot2, vec2 }, platform::{Time, Connection, Socket}, network::Packet,
};

use super::{Shield, Cooldown};

/// Component for a bubble shield.
pub struct BubbleShield {
    /// Radius that decreases over time
    pub radius: f32,
}

pub fn instantiate(world: &mut World, owner: Entity, binding: usize) -> Entity {
    world.spawn((
        Ability {
            owner,
            binding,
            active: false,
        },
        Sprite::new(Costume::BubbleShield {
            position: Default::default(),
            radius: Default::default(),
        }),
        BubbleShield { radius: 50.0 },
        Shield(owner),
        Cooldown(0.0),
        Transform::default(),
        Collider::circle(50.0),
        LocalPosition(vec2!(0.0, 0.0)),
    ))
}

/// System that positions the shield
pub fn bubble_shield_controller(world: &mut World, socket: &Socket, time: &Time) {
    for (_, (ability, shield, cooldown, collider)) in &mut world.query::<(
        &Ability, &mut BubbleShield, &mut Cooldown, &mut Collider
    )>() {
        // Cooldown
        cooldown.0 -= time.dt();
        // Shrink shield
        if ability.active && cooldown.0 <= 0.0 {
            shield.radius -= 1.5 * time.dt();
            *collider = Collider::circle(shield.radius);
        }
        if shield.radius <= 15.0 {
            shield.radius = 50.0;
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
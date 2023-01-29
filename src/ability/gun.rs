use hecs::{World, Entity};

use crate::{
    math::Vec2,
    platform::Time,
    ability::Ability,
    transform::Transform,
    input::Input,
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
pub fn gun_controller(world: &mut World, time: &Time) {
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
        if ability.active && cooldown.0 <= 0.0 && input.button(0) {
            shots.push((gun.shoot, ability.owner, transform.translation, input.look_axis()));
            *cooldown = gun.cooldown;
        }
    }
    for (shoot, e, o, v) in shots {
        (shoot)(world, e, o, v);
    }
}
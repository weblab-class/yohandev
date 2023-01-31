use hecs::{World, Entity, With};

use crate::{
    ability::{ Ability, Cooldown }, input::Input, platform::Time, transform::Transform, physics::KinematicBody,
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
pub fn push_controller(world: &mut World, time: &Time) {
    if cfg!(client) {
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
        }
    }
    for origin in pushes {
        for (_, (t, kb)) in world.query_mut::<(&Transform, &mut KinematicBody)>() {
            if let Some(delta) = (t.translation - origin).try_normalize(0.01) {
                kb.velocity = 2000.0 * delta;
            }
        }
    }
}
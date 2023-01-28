use hecs::{World, Entity, EntityBuilder};

use crate::{math::Vec2, transform::Transform, vec2};

pub enum AbilityKind {
    Shotgun,
    AssaultRifle,
    DualGun,
    Shield,
}

pub struct Ability {
    /// Kind of ability,
    pub kind: AbilityKind,
    /// Parent entity
    pub owner: Entity,
    /// Ability active this frame?
    pub active: bool,
}

// Shotgun:
//  - Ability
//  - Sprite
//  - Transform
//  - Shotgun
//
//  - system that toggles on/off active for [Ability]'s
//  - system that sets the shotgun's transform
//  - system that listens to [Input] and [Ability] to fire shotgun bullets

impl Shotgun {
    pub fn prefab(parent: Entity) -> EntityBuilder {
        let mut builder = EntityBuilder::new();

        builder.add_bundle((
            Shotgun { parent },
            Transform::default(),
        ));
        builder
    }
}

pub fn update_shotgun_position(world: &mut World) {
    for (e, (transform, shotgun)) in &mut world.query::<(&mut Transform, &Shotgun)>() {
        if e != shotgun.parent {
            let Ok(target) = (unsafe {
                // SAFETY:
                // Asserted this entity isn't parented to itself
                world.get_unchecked::<&Transform>(shotgun.parent)
            }) else {
                continue;
            };
            // Offset position
            let target = target.translation + vec2!(-5.0, 20.0);
            // Lightly damp
            transform.translation += 0.9 * (target - transform.translation);
        }
    }
}
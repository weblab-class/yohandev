use hecs::{ Entity, EntityBuilder, World };
use nalgebra::ComplexField;

use crate::{
    transform::{ Transform, Parent, LocalPosition },
    math::vec2,
    render::{ Sprite, Costume }, input::Input, bullet
};

/// Component that marks this entity as an ability
#[derive(Debug)]
pub struct Ability {
    /// Parent entity
    pub owner: Entity,
    /// Ability active this frame?
    pub active: bool,
}

/// Component that marks this entity as a shotgun ability
#[derive(Debug, Default)]
pub struct Shotgun;

pub fn shotgun_prefab(owner: Entity) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Shotgun,
        Ability {
            owner,
            // TODO: ability switcher, this should be false.
            active: true,
        },
        Sprite::new(Costume::Shotgun {
            position: Default::default(),
            rotation: Default::default(),
        }),
        Transform::default(),
        Parent::new(owner),
        LocalPosition(vec2!(-15.0, 20.0)),
    ));
    builder
}

/// System that does the shotgun functionality
pub fn shotgun_controller(world: &mut World) {
    /// Queries all weapon holders
    type Query<'a> = (
        &'a Ability,            // Needed to test if active or not
        &'a Shotgun,            // Marker to query shotguns only
        &'a mut Transform,      // Origin of bullets
    );
    let mut shots = Vec::new();
    for (_, (ability, _shotgun, transform)) in &mut world.query::<Query>() {
        // User input
        let Ok(input) = world.get::<&Input>(ability.owner) else {
            continue;
        };
        let dir = input.look_axis();
        transform.rotation = dir.y.atan2(dir.x);
        // TODO: networkrotation
        if cfg!(server) {
            // Shoot(TODO: cooldown)
            if ability.active && input.button(0) {
                shots.push((transform.translation, dir));
            }
        }
    }
    for (o, v) in shots {
        world.spawn(bullet::prefab(o, v).build());
    }
}

// Shotgun:
//  - Ability
//  - Sprite
//  - Transform
//  - Shotgun
//
//  - system that toggles on/off active for [Ability]'s
//  - system that listens to [Input] and [Ability] to fire shotgun bullets
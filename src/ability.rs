use hecs::{ Entity, EntityBuilder };

use crate::{
    transform::{ Transform, Parent, LocalPosition },
    math::vec2,
    render::{ Sprite, Costume }
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
        }),
        Transform::default(),
        Parent::new(owner),
        LocalPosition(vec2!(-5.0, 20.0)),
    ));
    builder
}

// Shotgun:
//  - Ability
//  - Sprite
//  - Transform
//  - Shotgun
//
//  - system that toggles on/off active for [Ability]'s
//  - system that listens to [Input] and [Ability] to fire shotgun bullets
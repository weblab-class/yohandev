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
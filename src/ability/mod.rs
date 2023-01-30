pub use gun::*;

use hecs::{ Entity, EntityBuilder };

mod gun;
mod shotgun;
mod rifle;

/// Complete enumeration of all ability types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AbilityKind {
    Shotgun,
    AssaultRifle,
    DualGun,
    Shield,
}

/// Component that marks this entity as an ability
#[derive(Debug)]
pub struct Ability {
    /// Parent entity
    pub owner: Entity,
    /// Ability active this frame?
    pub active: bool,
}

pub fn prefab(owner: Entity, kind: AbilityKind) -> EntityBuilder {
    match kind {
        AbilityKind::Shotgun => shotgun::prefab(owner),
        AbilityKind::AssaultRifle => rifle::prefab(owner),
        AbilityKind::DualGun => todo!(),
        AbilityKind::Shield => todo!(),
    }
}
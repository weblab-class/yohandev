pub use gun::*;

use hecs::{ Entity, EntityBuilder, World };

use crate::{platform::Socket, input::Input, network::Packet};

mod gun;
mod shotgun;
mod rifle;
mod pistols;

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
    /// Binding slot 0-3
    pub binding: usize,
    /// Ability active this frame?
    pub active: bool,
}

pub fn prefab(owner: Entity, binding: usize, kind: AbilityKind) -> EntityBuilder {
    match kind {
        AbilityKind::Shotgun => shotgun::prefab(owner, binding),
        AbilityKind::AssaultRifle => rifle::prefab(owner, binding),
        AbilityKind::DualGun => pistols::prefab(owner, binding),
        AbilityKind::Shield => todo!(),
    }
}

/// System that toggles on/off abilities
pub fn toggle_abilities(world: &mut World, socket: &Socket) {
    // Server toggles abilities
    if cfg!(server) {
        for (e, input) in &mut world.query::<&Input>() {
            // Chosen ability
            let chosen = (0..4).find(|&i| input.button(i));

            for (_, ability) in &mut world.query::<&mut Ability>() {
                if ability.owner != e {
                    continue;
                }
                // At most 1 ability at a time
                ability.active = chosen
                    .filter(|&i| ability.binding == i)
                    .is_some();
            }
            // TODO: upload only when changed, reliably
            socket.broadcast(&Packet::PlayerToggleAbility(e, chosen));
        }
    }
    // Client just listens
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::PlayerToggleAbility(owner, binding) = packet else {
                continue;
            };
            for (_, ability) in &mut world.query::<&mut Ability>() {
                if ability.owner != *owner {
                    continue;
                }
                ability.active = binding
                    .filter(|&i| ability.binding == i)
                    .is_some();
            }
        }
    }
}
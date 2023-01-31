pub use gun::*;
pub use shield::{ Shield, position_shield };
pub use push::push_controller;

use hecs::{ Entity, World };

use crate::{platform::Socket, input::Input, network::Packet};

mod gun;
mod shotgun;
mod rifle;
mod pistols;
mod shield;
mod push;

/// Complete enumeration of all ability types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AbilityKind {
    Shotgun,
    AssaultRifle,
    DualGun,
    Shield,
    Push,
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

pub fn instantiate(world: &mut World, owner: Entity, binding: usize, kind: AbilityKind) -> Entity {
    match kind {
        AbilityKind::Shotgun => shotgun::instantiate(world, owner, binding),
        AbilityKind::AssaultRifle => rifle::instantiate(world, owner, binding),
        AbilityKind::DualGun => pistols::instantiate(world, owner, binding),
        AbilityKind::Shield => shield::instantiate(world, owner, binding),
        AbilityKind::Push => push::instantiate(world, owner, binding),
    }
}

/// System that toggles on/off abilities
pub fn toggle_abilities(world: &mut World, socket: &Socket) {
    // Server toggles abilities
    if cfg!(server) {
        for (e, input) in &mut world.query::<&Input>() {
            // Chosen ability
            let chosen = (0..4).find(|&i| input.ability(i));

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
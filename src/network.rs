use hecs::Entity;

use crate::{
    input::Input,
    math::Vec2,
    platform::Connection,
};

/// Server <-> Client messages.
/// TODO: serialize these since each variant has differnet width, the
/// raw byte repr is not suitable over the wire
#[derive(Debug, Clone)]
pub enum Packet {
    /// Server -> Clients
    PlayerSpawn(Entity, Connection),
    /// Server -> Clients
    PlayerDespawn(Entity),
    /// Client -> Server
    PlayerCommand(Input),
    /// Server -> Clients
    EntityPosition(Entity, Vec2<f32>),
    /// Server -> Clients
    ProjectileSpawn {
        origin: Vec2<f32>,
        velocity: Vec2<f32>,
    }
}
use hecs::Entity;
use vek::Vec2;

use crate::{
    input::Input,
    spawn::Prefab
};

/// Marker component that indicates an entity should be replicated
/// over the network.
pub struct Networked;

/// Server <-> Client messages.
/// TODO: serialize these since each variant has differnet width, the
/// raw byte repr is not suitable over the wire
#[derive(Debug, Clone)]
#[allow(unused)]
#[repr(u8)]
pub enum Packet {
    /// Server -> Clients
    EntitySpawn(Entity, Prefab),
    /// Server -> Clients
    EntityPosition(Entity, Vec2<f32>),
    /// Client -> Server
    PlayerCommand(Input),
}
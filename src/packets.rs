use hecs::Entity;
use vek::Vec2;

use crate::{platform::Connection, input::Input};

/// Server <-> Client messages.
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Packet {
    Ping,
    Pong,
    /// Server -> Clients
    SpawnPlayer {
        /// ID to spawn
        entity: Entity,
        /// Connection of that client
        connection: Connection,
    },
    /// Server -> Clients
    EntityPosition {
        /// Target entity
        entity: Entity,
        /// New absolute position(not delta encoded)
        position: Vec2<f32>,
    },
    /// Client -> Server
    Input(Input),
}
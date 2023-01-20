use hecs::Entity;

use crate::platform::Connection;

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
    }
}
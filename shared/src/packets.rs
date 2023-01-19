use hecs::Entity;

use crate::input::Input;

#[derive(Debug, Clone, Copy)]
pub enum Packet {
    Ping,
    Pong,
    /// (S -> C) Get which entity is owned by you(client).
    WhoAmI(Entity),
    /// (S -> C) Spawn a new player, including (maybe) your own.
    PlayerSpawn {
        ent: Entity,
    },
    /// (C -> S) Snapshot of a client's input.
    PlayerInput {
        // TODO: add time of snapshot ie:
        // https://gafferongames.com/post/networked_physics_2004/
        state: Input,
    },
}
use hecs::Entity;

#[derive(Debug, Clone, Copy)]
pub enum Packet {
    Ping ,
    Pong,
    SpawnPlayer {
        ent: Entity
    },
}
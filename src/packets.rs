/// Server <-> Client messages.
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Packet {
    Ping,
    Pong,
}
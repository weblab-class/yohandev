/// Server <-> Client messages.
#[repr(u8)]
pub enum Packet {
    Ping,
    Pong,
}
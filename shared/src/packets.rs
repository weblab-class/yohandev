#[derive(Debug, Clone, Copy)]
pub enum Packet {
    Ping = 0x0,
    Pong = 0x1,
}
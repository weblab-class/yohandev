#[derive(Clone, Copy)]
pub enum Packet {
    /// "Termination" packet used to signify the end of a
    /// packet stream over FFI.
    None = 0x0,
    Ping = 0x1,
    Pong = 0x2,
}
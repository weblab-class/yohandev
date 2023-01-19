/// Snapshot of a player input. Used as a component
/// and a packet payload.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Input {
    /// X direction of movement.
    pub dx: i8,
    /// Y direction of movement.
    pub dy: i8,
}
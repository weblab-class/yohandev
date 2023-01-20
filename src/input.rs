use hecs::World;

use crate::platform::Gamepad;

/// Snapshot of a player's input. Used as both a
/// component and a packet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Input {
    /// Movement in the X direction(quantized)
    dx: i8,
    /// Movement in the Y direction(quantized)
    dy: i8,
}

impl Input {
    /// Movement in the X direction, `-1.0..=1.0`
    pub fn dx(&self) -> f32 {
        const MAX: i8 = i8::MAX;
        // Rectify left-leaning `i8`
        self.dx.max(-MAX) as f32 / MAX as f32
    }

    /// Movement in the Y direction, `-1.0..=1.0`
    pub fn dy(&self) -> f32 {
        const MAX: i8 = i8::MAX;
        // Rectify left-leaning `i8`
        self.dy.max(-MAX) as f32 / MAX as f32
    }
}

/// `Client system`:
///     - Updates the local player's input component.
///     - Sends input snapshots to the server.
/// 
/// `Server system`:
///     - Listens for client's incoming inputs.
///     - Updates respective entities' input components.
pub fn update(world: &mut World, gamepad: &Gamepad) {
    const MAX: f32 = i8::MAX as _;

    for (_, input) in world.query_mut::<&mut Input>() {
        if cfg!(client) {
            input.dx = (MAX * gamepad.dx()) as _;
            input.dy = (MAX * gamepad.dy()) as _;
        }
    }
}
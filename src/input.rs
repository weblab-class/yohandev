/// Snapshot of a player's input. Used as both a
/// component and a packet.
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
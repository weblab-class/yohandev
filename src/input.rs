use hecs::{World, PreparedQuery};

use crate::{platform::{Gamepad, Socket, Connection}, packets::Packet};

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

/// Client system that polls user inputs and updates them
/// on the client.
pub fn update(world: &mut World, gamepad: &Gamepad) {
    const MAX: f32 = i8::MAX as _;

    if cfg!(server) {
        return;
    }

    for (_, input) in world.query_mut::<&mut Input>() {
        input.dx = (MAX * gamepad.dx()) as _;
        input.dy = (MAX * gamepad.dy()) as _;
    }
}

/// System that synchronizes the `Input` component over the
/// network.
pub fn sync(world: &mut World, socket: &Socket) {
    if cfg!(client) {
        for (_, &input) in world.query_mut::<&Input>() {
            socket.broadcast(&Packet::Input(input));
        }
    }
    if cfg!(server) {
        // Query to search for the relevant entity
        let mut query = PreparedQuery::<(&Connection, &mut Input)>::default();

        for (connection, packet) in socket.packets() {
            if let Packet::Input(input) = packet {
                query
                    .query_mut(world)
                    .find(|(_, (conn, _))| *conn == connection)
                    .map(|(_, (_, inp))| {
                        *inp = *input;
                    });
            }
        }
    }
}
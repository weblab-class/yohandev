use hecs::World;

use crate::{
    platform::{ Gamepad, Socket },
    network::Packet
};

/// Snapshot of a player's input. Used as both a
/// component and a packet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Input {
    /// Movement in the X direction(quantized)
    dx: i8,
    /// Movement in the Y direction(quantized)
    dy: i8,
    /// Ability buttons(bitfield)
    btn: u8,
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

    /// Get the `ith` button input
    pub fn button(&self, i: usize) -> bool {
        (self.btn & (1 << i)) != 0
    }
}

/// Client system that polls user inputs and updates them on the client.
pub fn update(world: &mut World, gamepad: &Gamepad) {
    const MAX: f32 = i8::MAX as _;

    if cfg!(server) {
        return;
    }

    for (_, input) in world.query_mut::<&mut Input>() {
        input.dx = (MAX * gamepad.dx()) as _;
        input.dy = (MAX * gamepad.dy()) as _;
        input.btn = 0;

        for i in 0..8 {
            input.btn |= (gamepad.button(i) as u8) << i;
        }
    }
}

/// System that synchronizes the `Input` component over the network.
#[cfg(client)]
pub fn network_player_commands(world: &mut World, socket: &Socket) {
    for (_, &input) in world.query_mut::<&Input>() {
        socket.broadcast(&Packet::PlayerCommand(input));
    }
}

#[cfg(server)]
pub fn network_player_commands(world: &mut World, socket: &Socket) {
    use crate::platform::Connection;

    /// Query to find entity the input corresponds to.
    type Query<'a> = (&'a mut Input, &'a Connection);

    for (connection, packet) in socket.packets() {
        let Packet::PlayerCommand(command) = packet else {
            continue;
        };
        if let Some((_, (input, _))) = world
            .query_mut::<Query>()
            .into_iter()
            .find(|(_, (_, c))| *c == connection) {
                *input = *command;
        }
    }
}
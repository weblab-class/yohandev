use hecs::{World, With};

use crate::{
    platform::{ Gamepad, Socket },
    network::Packet, transform::Transform,
    math::{ Vec2, vec2 }, player::Player,
};

/// Snapshot of a player's input. Used as both a
/// component and a packet.
#[derive(Debug, Default, Clone, Copy)]
pub struct Input {
    /// Movement in the X direction(quantized)
    dx: i8,
    /// Movement in the Y direction(quantized)
    dy: i8,
    /// Attack direction X component(quantized)
    ax: i8,
    /// Attack direction Y component(quantized)
    ay: i8,
    /// Ability buttons(bitfield)
    btn: u8,
}

impl Input {
    /// Maximum magnitude for quantized axes.
    /// Used to rectify `i8` which is left-leaning
    const MAX: i8 = i8::MAX;

    /// Movement in the X direction, `-1.0..=1.0`
    pub fn dx(&self) -> f32 {
        self.dx.max(-Self::MAX) as f32 / Self::MAX as f32
    }

    /// Movement in the Y direction, `-1.0..=1.0`
    pub fn dy(&self) -> f32 {
        self.dy.max(-Self::MAX) as f32 / Self::MAX as f32
    }

    /// Attack direction X component, `-1.0..=1.0`
    pub fn ax(&self) -> f32 {
        self.ax.max(-Self::MAX) as f32 / Self::MAX as f32
    }

    /// Attack direction Y component, `-1.0..=1.0`
    pub fn ay(&self) -> f32 {
        self.ay.max(-Self::MAX) as f32 / Self::MAX as f32
    }

    /// Get the `ith` button input
    pub fn button(&self, i: usize) -> bool {
        (self.btn & (1 << i)) != 0
    }

    pub fn move_axis(&self) -> Vec2<f32> {
        vec2!(self.dx(), self.dy())
    }

    pub fn look_axis(&self) -> Vec2<f32> {
        vec2!(self.ax(), self.ay())
    }
}

/// Client system that polls user inputs and updates them on the client.
pub fn update(world: &mut World, gamepad: &Gamepad) {
    const MAX: f32 = i8::MAX as _;

    if cfg!(server) {
        return;
    }
    // Construct input component(same for everyone)
    let new = Input {
        dx: (MAX * gamepad.dx()) as _,
        dy: (MAX * gamepad.dy()) as _,
        ax: (MAX * gamepad.ax()) as _,
        ay: (MAX * gamepad.ay()) as _,
        btn: (0..8)
            .map(|i| (gamepad.button(i) as u8) << i)
            .fold(0, |accum, btn| accum | btn),
    };

    for (_, input) in world.query_mut::<&mut Input>() {
        *input = new;
    }
    for (_, t) in world.query_mut::<With<&Transform, (&Player, &Input)>>() {
        gamepad.set_player_position(t.translation.x, t.translation.y);
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
use shared::{
    ecs::{ World, With },
    Packet,
    Input,
};
use vek::Vec2;

use crate::draw::Sprite;
use crate::net;

/// Component tagging this entity as client owned.
pub struct OwnedPlayer;

/// System that spawns networked player entities.
pub fn spawn(world: &mut World, packet: &Packet) {
    if let Packet::PlayerSpawn { ent } = packet {
        world.spawn_at(*ent, (
            Sprite::Circle(Vec2::new(0.0, 0.0), 10.0),
        ));
    }
}

/// System that sends a player's input to the server.
pub fn input(world: &World) {
    for (_, &state) in &mut world.query::<With<&Input, &OwnedPlayer>>() {
        net::send(&Packet::PlayerInput { state });
    }
}
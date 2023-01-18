use vek::Vec2;
use hecs::World;
use shared::Packet;

use crate::draw::Sprite;

/// System that spawns networked player entities.
pub fn players(world: &mut World, packet: &Packet) {
    if let Packet::SpawnPlayer { ent } = packet {
        world.spawn_at(*ent, (
            Sprite::Circle(Vec2::new(0.0, 0.0), 10.0),
        ));
    }
}
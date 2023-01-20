use hecs::World;
use vek::Vec2;

use crate::{platform::Socket, packets::Packet};

/// Component for an entity's global transform.
#[derive(Debug, Default)]
pub struct Transform {
    /// Position
    pub translation: Vec2<f32>,
    /// CCW
    pub rotation: f32,
}

/// Component that tags an entity's `Transform::translation` as
/// synchronized over the network.
#[derive(Debug, Default)]
pub struct NetPosition {
    /// Last sent/received position over the network
    last: Vec2<f32>,
}

// TODO: LocalPos, LocalRot and etc. systems

/// System that synchronizes entity positions over the
/// network.
pub fn sync_position(world: &mut World, socket: &Socket) {
    if cfg!(server) {
        type Query<'a> = (&'a Transform, &'a mut NetPosition);

        for (entity, (transform, pos)) in world.query_mut::<Query>() {
            // Skip entities that don't move.
            if pos.last == transform.translation {
                continue;
            }
            pos.last = transform.translation;

            // Let clients know
            socket.broadcast(&Packet::EntityPosition {
                entity,
                position: transform.translation,
            });
        }
    }
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            if let Packet::EntityPosition { entity, position } = packet {
                if let Ok(mut transform) = world.get::<&mut Transform>(*entity) {
                    // TODO: update last position
                    transform.translation = *position;
                }
            }
        }
    }
}
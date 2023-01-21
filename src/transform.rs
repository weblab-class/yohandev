use hecs::World;

use crate::{
    platform::Socket,
    network::Packet,
    math::Vec2,
};

/// Component for an entity's global transform.
#[derive(Debug, Default)]
pub struct Transform {
    /// Position
    pub translation: Vec2<f32>,
    /// CCW
    pub rotation: f32,
}

// TODO: LocalPos, LocalRot and etc. systems
// TODO: Networked position buffer

/// System that synchronizes entity positions over the network.
#[cfg(server)]
pub fn networked_position(world: &mut World, socket: &Socket) {
    use crate::network::Networked;

    type Query<'a> = hecs::With<&'a Transform, &'a Networked>;

    for (e, transform) in world.query_mut::<Query>() {
        socket.broadcast(
            &Packet::EntityPosition(e, transform.translation)
        );
    }
}

#[cfg(client)]
pub fn networked_position(world: &mut World, socket: &Socket) {
    for (_, packet) in socket.packets() {
        let Packet::EntityPosition(e, position) = packet else {
            continue;
        };
        if let Ok(mut transform) = world.get::<&mut Transform>(*e) {
            transform.translation = *position;
        }
    }
}
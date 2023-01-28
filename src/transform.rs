use nalgebra::Isometry2;
use hecs::{ World, With, Entity };

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

impl From<&Transform> for Isometry2<f32> {
    fn from(t: &Transform) -> Self {
        Self::new(t.translation, t.rotation)
    }
}

/// Marker trait that an entity's position should be replicated.
#[derive(Debug, Default)]
pub struct NetworkPosition;

/// Component that store's an entity's parent
#[derive(Debug)]
pub struct Parent {
    /// Entity that is the parent
    pub handle: Entity,
    /// Damping factor for child to follow(1.0 = none)
    pub damping: f32,
}

// TODO: LocalPos, LocalRot and etc. systems
// TODO: Networked position buffer

/// System that synchronizes entity positions over the network.
#[cfg(server)]
pub fn networked_position(world: &mut World, socket: &Socket) {
    type Query<'a> = With<&'a Transform, &'a NetworkPosition>;

    for (e, t) in world.query_mut::<Query>() {
        socket.broadcast(&Packet::EntityPosition(e, t.translation));
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

/// System that updates children's positions according to their [Parent]
pub fn update_children(world: &mut World) {
    for (e, (transform, parent)) in &mut world.query::<(&mut Transform, &Parent)>() {
        if e != parent.handle {
            let Ok(target) = (unsafe {
                // SAFETY:
                // Asserted this entity isn't parented to itself
                world.get_unchecked::<&Transform>(parent.handle)
            }) else {
                continue;
            };
            transform.translation += parent.damping * (target.translation - transform.translation);
        }
    }
}
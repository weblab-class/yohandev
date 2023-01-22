use hecs::{ EntityBuilder, World };

use crate::{
    transform::Transform,
    platform::Socket,
    network::{ Networked, Packet },
    render::Sprite,
    input::Input,
    physics::{ Collider, Collisions, KinematicBody, Gravity, Grounded },
    math::vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefab {
    Player,
}

impl Prefab {
    pub fn instantiate(&self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();

        match self {
            Self::Player => builder.add_bundle((
                Networked,
                Sprite::Rect,
                Input::default(),
                Collider::rect(20.0, 50.0),
                Collisions::default(),
                KinematicBody::default(),
                Grounded::default(),
                Gravity { acceleration: vec2!(0.0, -2500.0) },
                Transform {
                    translation: vec2!(0.0, 200.0),
                    rotation: 0.0,
                },
            )),
        };
        // Keep track of the parent prefab
        builder.add(*self);
        // Used by `networked_instantiate`
        if cfg!(server) && builder.has::<Networked>() {
            builder.add(NotYetReplicated);
        }
        builder
    }
}

/// Marker component for entities which have been instantiated on
/// the server but not yet replicated on clients.
struct NotYetReplicated;

/// System that synchronizes instantiations of prefabs.
#[cfg(server)]
pub fn networked_instantiate(world: &mut World, socket: &Socket) {
    use hecs::{ With, Without };
    /// Queries prefab'ed entities that have *not* yet been replicated.
    type Replicate<'a> = With<&'a Prefab, (&'a NotYetReplicated, &'a Networked)>;
    /// The remaining networked entities that *have* been replicated
    type Rest<'a> = Without<With<&'a Prefab, &'a Networked>, &'a NotYetReplicated>;

    let mut spawned = Vec::new();
    // Notify existing clients
    for (e, &prefab) in world.query_mut::<Replicate>() {
        // TODO: reliable
        socket.broadcast(&Packet::EntitySpawn(e, prefab));
        spawned.push(e);
    }
    // Notify new clients(with the rest of the world state)
    for (e, &prefab) in world.query_mut::<Rest>() {
        for &c in socket.connections() {
            // TODO: reliable
            socket.send(c, &Packet::EntitySpawn(e, prefab));
        }
    }
    // Mark entities as replicated
    for e in spawned {
        world.remove_one::<NotYetReplicated>(e).unwrap();
    }
}

#[cfg(client)]
pub fn networked_instantiate(world: &mut World, socket: &Socket) {
    for (_, packet) in socket.packets() {
        let Packet::EntitySpawn(e, prefab) = packet else {
            continue;
        };
        world.spawn_at(*e, prefab.instantiate().build());
    }
}
use hecs::{ EntityBuilder, World, With, Without };

use crate::{
    transform::Transform,
    platform::Socket,
    network::{ Networked, Packet },
    render::Sprite, input::Input
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefab {
    Player,
}

/// Marker component for entities which have been instantiated on
/// the server but not yet replicated on clients.
struct NotYetReplicated;

impl Prefab {
    pub fn instantiate(&self) -> EntityBuilder {
        let mut builder = EntityBuilder::new();

        match self {
            Self::Player => builder.add_bundle((
                Networked,
                Sprite::Rect,
                Input::default(),
                Transform::default(),
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

/// System that synchronizes instantiations of prefabs.
#[cfg(server)]
pub fn networked_instantiate(world: &mut World, socket: &Socket) {
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
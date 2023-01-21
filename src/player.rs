use hecs::World;

use crate::{
    physics::KinematicBody,
    input::Input,
    platform::Socket,
    spawn::Prefab,
};

/// System that updates player controllers.
pub fn controller(world: &mut World) {
    /// Queries all players
    type Query<'a> = (&'a mut KinematicBody, &'a Input);

    for (_, (kb, input)) in world.query_mut::<Query>() {
        kb.velocity.x = input.dx() * 100.0;
        if input.dy() > 0.0 {
            kb.velocity.y += 50.0;
        }
    }
}

/// System that instantiates a player entity for every client.
pub fn instantiate(world: &mut World, socket: &Socket) {
    if cfg!(client) {
        // TODO: send player "owned" entity ID for client-side prediction
        return;
    }
    for c in socket.connections() {
        world.spawn(
            Prefab::Player
                .instantiate()
                .add(*c)
                .build()
        );
    }
}
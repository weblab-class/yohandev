use hecs::World;

use crate::{
    physics::{ KinematicBody, Grounded },
    input::Input,
    platform::{ Socket, Time },
    spawn::Prefab,
};

/// System that updates player controllers.
pub fn controller(world: &mut World, time: &Time) {
    /// Queries all players
    type Query<'a> = (
        &'a mut KinematicBody,
        &'a Grounded,
        &'a Input
    );

    for (_, (kb, grounded, input)) in world.query_mut::<Query>() {
        // Movement
        kb.velocity.x +=  100.0 * input.dx() * time.dt();
        // Jump
        if matches!(grounded, Grounded::Yes { .. }) && input.dy() > 0.0 {
            kb.velocity.y += 250.0;
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
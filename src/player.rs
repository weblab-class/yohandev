use hecs::World;

use crate::{
    transform::Transform,
    input::Input,
    platform::Socket,
    spawn::Prefab,
    math::vec2,
};

/// System that updates player controllers.
pub fn controller(world: &mut World) {
    /// Queries all players
    type Query<'a> = (&'a mut Transform, &'a Input);

    for (_, (transform, input)) in world.query_mut::<Query>() {
        // TODO: use delta time
        transform.translation += vec2!(input.dx(), input.dy())
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
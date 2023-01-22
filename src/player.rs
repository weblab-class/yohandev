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
    // TODO: these will be calculated from player abilities
    const SPEED: f32 = 1200.0;
    const JUMP: f32 = 1500.0;
    const JUMP_GRACE_PERIOD: f32 = 0.1;
    const JUMP_TERM_VELOCITY: f32 = 500.0;
    const FRICTION: f32 = 5.0;

    for (_, (kb, grounded, input)) in world.query_mut::<Query>() {
        // Movement
        kb.velocity.x +=  SPEED * input.dx() * time.dt();
        // Jump
        if input.dy() > 0.0 {
            let can_jump = match grounded {
                Grounded::Yes { .. } => true,
                Grounded::No { time } => {
                    // Allow short-while after falling off cliff...
                    *time <= JUMP_GRACE_PERIOD
                    // ...but not double jumping
                    && kb.velocity.y <= 0.0
                },
            };
            if can_jump {
                kb.velocity.y += JUMP;
            }
        // Jump termination
        } else if kb.velocity.y > JUMP_TERM_VELOCITY {
            kb.velocity.y = JUMP_TERM_VELOCITY;
        }
        // Damping
        kb.velocity /= 1.0 + FRICTION * time.dt();
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
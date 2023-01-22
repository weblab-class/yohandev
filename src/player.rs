use hecs::{World, EntityBuilder, With};

use crate::{
    physics::{ KinematicBody, Grounded, Collider, Collisions, Gravity },
    input::Input,
    platform::{ Socket, Time, Connection },
    render::Sprite,
    transform::{Transform, NetworkPosition},
    math::vec2, network::Packet, bullet,
};

/// Component that marks an entity as a player.
pub struct Player;

/// System that instantiates players over the network.
pub fn networked_instantiate(world: &mut World, socket: &Socket) {
    /// Prefab for a player entity
    fn prefab(builder: &mut EntityBuilder) -> &mut EntityBuilder {
        builder.add_bundle((
            Player,
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
            NetworkPosition::default(),
        ))
    }
    // Server spawns player for every connection
    if cfg!(server) {
        for &connection in socket.connections() {
            let e = world.spawn(
                prefab(&mut Default::default())
                    .add(connection)
                    .build()
            );
            // TODO: reliable transport
            socket.broadcast(&Packet::PlayerSpawn(e, connection));
            // Existing players
            for (e, &c) in world.query_mut::<With<&Connection, &Player>>() {
                socket.send(connection, &Packet::PlayerSpawn(e, c));
            }
        }
    }
    // Client spawns player whenever it's told so
    if cfg!(client) {
        for (connection, packet) in socket.packets() {
            let Packet::PlayerSpawn(e, c) = packet else {
                continue;
            };
            world.spawn_at(*e, prefab(&mut Default::default()).build());
            // Owned entity
            if connection != c {
                world.remove_one::<Input>(*e).unwrap();
            }
        }
    }
}

/// System that updates player controllers.
pub fn platformer_controller(world: &mut World, time: &Time) {
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

/// System that shoots projectiles
pub fn weapon_controller(world: &mut World) {
    /// Queries all weapon holders
    type Query<'a> = (
        &'a Transform,
        &'a Input,
    );
    let mut commands = Vec::new();
    for (_, (transform, input)) in world.query_mut::<Query>() {
        // Shoot
        if input.button(0) {
            commands.push((transform.translation, vec2!(200.0, 0.0)));
        }
    }
    for (o, v) in commands {
        world.spawn(bullet::prefab(o, v).build());
    }
}
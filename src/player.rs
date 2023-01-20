use hecs::{ World, With, EntityBuilder };
use vek::Vec2;

use crate::{
    transform::Transform,
    input::Input,
    platform::{ Socket, Connection },
    render::Sprite,
    packets::Packet
};

/// Component that marks an entity as player-owned.
pub struct Player;
/// Component that marks this player as mine(client).
pub struct OwnedPlayer;

/// System that updates player controllers.
pub fn controller(world: &mut World) {
    /// Queries all owned player
    type Query<'a> = With<(&'a mut Transform, &'a Input), &'a Player>;

    for (_, (transform, input)) in world.query_mut::<Query>() {
        // TODO: use delta time
        transform.translation += Vec2 {
            x: input.dx(),
            y: input.dy(),
        }
    }
}

/// System that spawns players.
pub fn spawn(world: &mut World, socket: &Socket) {
    // Player bundle
    let bundle = || (
        Player,
        Transform::default(),
        Sprite::Rect,
    );
    if cfg!(server) {
        // Spawn player entity when connected...
        for &connection in socket.connections() {
            // ...and notify clients:
            // TODO: reliable
            socket.broadcast(&Packet::SpawnPlayer {
                entity: world.spawn(
                    EntityBuilder::new()
                        .add_bundle(bundle())
                        .add(Input::default())
                        .add(connection)
                        .build()
                ),
                connection,
            });
            // 
        }
    }
    if cfg!(client) {
        // Spawn whenever server says so
        for (me, packet) in socket.packets() {
            if let Packet::SpawnPlayer { entity, connection } = packet {
                let mut builder = EntityBuilder::new();
                // Mark as owned
                if connection == me {
                    builder
                        .add(OwnedPlayer)
                        .add(Input::default());
                }
                world.spawn_at(
                    *entity,
                    builder
                        .add_bundle(bundle())
                        .add(*connection)
                        .build()
                );
            }
        }
    }
}

/// System that notifies a new connection of every existing player.
pub fn spawn_existing(world: &mut World, socket: &Socket) {
    if cfg!(client) {
        return;
    } 
    type Query<'a> = With<&'a Connection, &'a Player>;

    for &incoming in socket.connections() {
        for (entity, &connection) in &mut world.query::<Query>() {
            // TODO: reliable
            socket.send(incoming, &Packet::SpawnPlayer {
                entity,
                connection
            });
        }
    }
}
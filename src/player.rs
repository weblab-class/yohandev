use hecs::{ World, EntityBuilder, Entity };

use crate::{
    physics::{ KinematicBody, Grounded, Collider, Gravity, self },
    input::{Input, LookDirection},
    platform::{ Socket, Time, Connection },
    render::{ Sprite, Costume, Shadow },
    transform::{ Transform, NetworkPosition, Parent },
    math::vec2,
    network::Packet,
    ability::{ AbilityKind, self, Ability, TimeScale },
    health::{ Health, self }, bullet::TimeToLive,
};

/// Component that marks an entity as a player.
pub struct Player {
    deck: [AbilityKind; 4],
    color: usize,
}

/// Prefab for a player entity
fn prefab(deck: [AbilityKind; 4], color: usize) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    builder.add_bundle((
        Player { deck, color },
        Sprite::new(Costume::Player {
            position: vec2!(0.0, 500.0),
            scale: vec2!(1.0, 1.0),
            lean: 0.0,
            color: color as _,
        }),
        Health {
            now: 100.0,
            max: 100.0,
        },
        Input::default(),
        Collider::rect(30.0, 50.0),
        Grounded::default(),
        Gravity { acceleration: vec2!(0.0, -2500.0) },
        Transform {
            translation: vec2!(100.0, 500.0),
            rotation: 0.0,
        },
        NetworkPosition::default(),
        LookDirection::default(),
    ));
    if cfg!(server) {
        builder.add(KinematicBody::default());
    }
    builder
}

/// System that instantiates players over the network.
pub fn networked_instantiate(
    world: &mut World,
    socket: &Socket,
    reserved: &mut impl Iterator<Item = Entity>
) {
    if cfg!(server) {
        // Server spawns player for every connection
        for (connection, deck) in socket.joins() {
            let e = reserved.next().unwrap_or_else(|| world.reserve_entity());
            let color = e.id() as usize;
            // Player
            world.spawn_at(e, prefab(*deck, color).add(*connection).build());
            // Abilities
            for (i, kind) in deck.iter().enumerate() {
                ability::instantiate(world, e, i, *kind);
            }
            // TODO: reliable transport
            socket.broadcast(&Packet::PlayerSpawn(e, *connection, *deck, color));
        }
        // Synchronize world state with every new connection
        for &connection in socket.connections() {
            // Existing players
            for (e, (c, player)) in world.query_mut::<(&Connection, &Player)>() {
                // TODO: reliable transport
                socket.send(connection, &Packet::PlayerSpawn(e, *c, player.deck, player.color));
            }
        }
    }
    // Client spawns player whenever it's told so
    if cfg!(client) {
        for (connection, packet) in socket.packets() {
            let Packet::PlayerSpawn(e, c, deck, color) = packet else {
                continue;
            };
            // Player:
            world.spawn_at(*e, prefab(*deck, *color).build());
            // Health bar:
            world.spawn(health::gui_prefab(*e).build());
            // Abilities:
            for (i, kind) in deck.iter().enumerate() {
                ability::instantiate(world, *e, i, *kind);
            }
            // Shadow
            world.spawn((
                Shadow(*e),
                Sprite::new(Costume::Shadow {
                    position: Default::default(),
                    scale: Default::default(),
                }),
            ));
            // Spawn indicator
            instantiate_spawn_indicator(world, *e);
            // Owned entity
            if connection != c {
                world.remove_one::<Input>(*e).unwrap();
            }
        }
    }
}

pub fn networked_despawn(world: &mut World, socket: &Socket) {
    // Server despawns player for every connection
    if cfg!(server) {
        for connection in socket.disconnections() {
            let Some(e) = world
                .query_mut::<&Connection>()
                .into_iter()
                .find(|(_, c)| *c == connection)
                .map(|(e, _)| e) else {
                    continue;
                };
            world.despawn(e).unwrap();
            // Abilities
            let mut destroy = Vec::new();
            for (e2, ability) in world.query_mut::<&Ability>() {
                if ability.owner == e {
                    destroy.push(e2);
                }
            }
            for e in destroy {
                world.despawn(e).unwrap();
            }
            // TODO: reliable transport
            socket.broadcast(&Packet::PlayerDespawn(e));
        }
    }
    // Client despawns whatever it's told to
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::PlayerDespawn(e) = packet else {
                continue;
            };
            if let Err(_) = world.despawn(*e) {
                log::warn!("Server tried to despawn a dead entity");
            }
            // Abilities
            let mut destroy = Vec::new();
            for (e2, ability) in world.query_mut::<&Ability>() {
                if ability.owner == *e {
                    destroy.push(e2);
                }
            }
            // Healthbar
            for (e2, (parent, sprite)) in world.query_mut::<(&Parent, &Sprite)>() {
                let Costume::HealthBar { .. } = &sprite.costume else {
                    continue;
                };
                if parent.0 == *e {
                    destroy.push(e2);
                }
            }
            for e in destroy {
                world.despawn(e).unwrap();
            }
        }
    }
}

pub fn instantiate_spawn_indicator(world: &mut World, entity: Entity) {
    let ground = {
        let Ok(transform) = world.get::<&Transform>(entity) else {
            return;
        };
        let Some((_, ground)) = physics::raycast_solid(
            world,
            transform.translation,
            vec2!(0.0, -1.0),
            Some(entity),
        ) else {
            return;
        };
        ground
    };
    world.spawn((
        Sprite::new(Costume::SpawnIn { position: ground }),
        TimeToLive::Frames(200),
    ));
}

/// System that updates player controllers.
pub fn platformer_controller(world: &mut World, time: &Time) {
    /// Queries all players
    type Query<'a> = (
        &'a mut KinematicBody,
        &'a Grounded,
        &'a Input,
        Option<&'a TimeScale>,
    );
    // TODO: these will be calculated from player abilities
    const SPEED: f32 = 1700.0;
    const JUMP: f32 = 1500.0;
    const JUMP_GRACE_PERIOD: f32 = 0.1;
    const JUMP_TERM_VELOCITY: f32 = 500.0;
    const FRICTION: f32 = 5.0;

    for (_, (kb, grounded, input, scale)) in world.query_mut::<Query>() {
        let scale = scale
            .map(|s| s.0)
            .unwrap_or(1.0);
        // Movement
        kb.velocity.x +=  SPEED * input.dx() * time.dt() * scale;
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
        kb.velocity /= 1.0 + FRICTION * time.dt() * scale;
    }
}
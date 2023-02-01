use hecs::{ World, Entity };

use crate::{
    ability::{ Ability, Cooldown },
    platform::{Time, Socket},
    transform::Transform,
    render::{ Sprite, Costume },
    network::Packet,
    physics::{self, Collider},
    math::vec2, bullet::TimeToLive,
};

/// Component that marks this entity as the el thor ability
enum Lightning {
    /// State machine: none, no lightning yet
    None,
    /// State machine: lightning is visually "loading" for `self.0` more seconds.
    Loading {
        time_left: f32,
        entity: Entity,
    },
    /// State machine: lightning has physical impact for `self.0` more seconds.
    Active {
        time_left: f32,
        entity: Entity,
    },
}

pub fn instantiate(world: &mut World, owner: Entity, binding: usize) -> Entity {
    world.spawn((
        Ability {
            owner,
            binding,
            active: false,
        },
        Lightning::None,
        Transform::default(),
        Cooldown::default(),
    ))
}

/// System that controls the lightning ability
pub fn lightning_controller(world: &mut World, time: &mut Time, socket: &Socket) {
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::EffectSpawn(costume) = packet else {
                continue;
            };
            if !matches!(costume, Costume::Lightning { .. }) {
                continue;
            }
            log::info!("spawn lightning");
            // Add sprite
            world.spawn((
                Sprite::new(costume.clone()),
                TimeToLive::Seconds(5.0),
            ));
        }
        return;
    }
    let mut add = Vec::new();
    for (_, (ability, cooldown, state)) in &mut world.query::<(&Ability, &mut Cooldown, &mut Lightning)>() {
        // Cooldown
        cooldown.0 -= time.dt();
        // State machine
        *state = match state {
            Lightning::None => {
                // Trigger
                if ability.active && cooldown.0 <= 0.0 {
                    *cooldown = Cooldown(5.0);
                    let entity = world.reserve_entity();
                    if let Ok(transform) = world.get::<&Transform>(ability.owner) {
                        add.push((ability.owner, entity, transform.translation));
                    }
                    // Transition
                    Lightning::Loading { time_left: 3.0, entity, }
                } else {
                    Lightning::None
                }
            },
            Lightning::Loading { time_left, entity } => {
                // Transition
                if *time_left <= 0.0 {
                    // TODO: give entity a damage component here
                    Lightning::Active {
                        time_left: 0.5,
                        entity: *entity,
                    }
                } else {
                    Lightning::Loading {
                        time_left: *time_left - time.dt(),
                        entity: *entity,
                    }
                }
            },
            Lightning::Active { time_left, entity } => {
                if *time_left <= 0.0 {
                    // TODO: delete entity here
                    Lightning::None
                } else {
                    Lightning::Active {
                        time_left: *time_left - time.dt(),
                        entity: *entity,
                    }
                }
            },
        };
    }
    // Populate reserved entities
    for (owner, reserved, position) in add {
        // Impacton the ground
        let position = physics::raycast(world, position, vec2!(0.0, -1.0), Some(owner))
            .map(|(_, p)| p)
            .unwrap_or(position);
        world.spawn_at(reserved, (
            Transform {
                // account for center
                translation: position + vec2!(0.0, 2500.0),
                rotation: 0.0
            },
            Collider::rect(100.0, 5000.0),
        ));
        socket.broadcast(&Packet::EffectSpawn(Costume::Lightning { position }));
    }
}
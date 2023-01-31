use hecs::{ World, Entity, With };

use crate::{
    ability::{ Ability, Cooldown },
    platform::{Time, Socket},
    transform::Transform,
    render::{ Sprite, Costume }, bullet::TimeToLive, network::Packet,
};

/// Component that marks this entity as the push ability
struct Freeze {
    /// Frames left to freeze
    frames: Option<usize>
}

/// Component for an entity's invidual time scale.
pub struct TimeScale(pub f32);

pub fn instantiate(world: &mut World, owner: Entity, binding: usize) -> Entity {
    world.spawn((
        Ability {
            owner,
            binding,
            active: false,
        },
        Freeze { frames: None },
        Cooldown::default(),
    ))
}

/// System that controls the almighty push
pub fn freeze_controller(world: &mut World, time: &mut Time, socket: &Socket) {
    const SCALE: f32 = 0.3;
    if cfg!(client) {
        for (_, packet) in socket.packets() {
            let Packet::EffectSpawn(costume) = packet else {
                continue;
            };
            if !matches!(costume, Costume::Freeze) {
                continue;
            }
            // Add sprite
            world.spawn((
                Sprite::new(costume.clone()),
                TimeToLive::Frames(100)
            ));
        }
        return;
    }
    let mut add = Vec::new();
    let mut remove = Vec::new();
    for (_, (ability, cooldown, freeze)) in &mut world.query::<(&Ability, &mut Cooldown, &mut Freeze)>() {
        // Cooldown
        cooldown.0 -= time.dt();
        // Trigger
        if ability.active && cooldown.0 <= 0.0 {
            time.scale = SCALE;
            freeze.frames = Some(240);
            *cooldown = Cooldown(5.0);
            add.push(ability.owner);
        }
        if let Some(frames) = &mut freeze.frames {
            *frames -= 1;
            if *frames <= 0 {
                time.scale = 1.0;
                remove.push(ability.owner);
            }
        }
    }
    for e in add {
        world.insert_one(e, TimeScale(1.0 / SCALE)).unwrap();
        // Sprite
        socket.broadcast(&Packet::EffectSpawn(Costume::Freeze));
    }
    for e in remove {
        world.remove_one::<TimeScale>(e).unwrap();
    }
}
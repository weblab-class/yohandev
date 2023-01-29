use hecs::{ Entity, EntityBuilder, World };
use nalgebra::Rotation2;

use crate::{
    transform::{ Transform, Parent, LocalPosition },
    math::{ Vec2, vec2 },
    render::{ Sprite, Costume }, input::{Input, FollowLookDirection}, bullet, platform::Time, health::Damage
};

/// Complete enumeration of all ability types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityKind {
    Shotgun,
    AssaultRifle,
    DualGun,
    Shield,
}

/// Component that marks this entity as an ability
#[derive(Debug)]
pub struct Ability {
    /// Parent entity
    pub owner: Entity,
    /// Ability active this frame?
    pub active: bool,
}

pub fn prefab(owner: Entity, kind: AbilityKind) -> EntityBuilder {
    match kind {
        AbilityKind::Shotgun => shotgun_prefab(owner),
        AbilityKind::AssaultRifle => todo!(),
        AbilityKind::DualGun => todo!(),
        AbilityKind::Shield => todo!(),
    }
}

/// Component for a generic gun's stats.
pub struct Gun {
    /// Gun's cooldown after each shot
    cooldown: Cooldown,
    /// Function that instantiates bullets 
    shoot: fn(world: &mut World, owner: Entity, origin: Vec2<f32>, velocity: Vec2<f32>),
}

/// Component for current cooldown time.
#[derive(Debug, Default, Clone, Copy)]
pub struct Cooldown(pub f32);

pub fn shotgun_prefab(owner: Entity) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Ability {
            owner,
            // TODO: ability switcher, this should be false.
            active: true,
        },
        Gun {
            // TODO these should come from `abilities.toml`
            cooldown: Cooldown(1.5),
            shoot: |world, owner, origin, velocity| {
                // TODO: this can be greatly optimized by simply sending the random seed
                for _ in 0..10 {
                    let spread = Rotation2::new(0.1 * (fastrand::f32() - 0.5));
                    let velocity = 1500.0 * (spread * velocity);
                    let damage = Damage {
                        amount: 5.0,
                        exclude: Some(owner),
                        destroy: true,
                    };
                    world.spawn(bullet::prefab(origin, velocity)
                        .add(damage)
                        .build()
                    );
                }
            },
        },
        Sprite::new(Costume::Shotgun {
            position: Default::default(),
            rotation: Default::default(),
        }),
        Cooldown::default(),
        Transform::default(),
        Parent(owner),
        FollowLookDirection(owner),
        LocalPosition(vec2!(0.0, 0.0)),
    ));
    builder
}

/// System that does the generic gun functionality
pub fn gun_controller(world: &mut World, time: &Time) {
    if cfg!(client) {
        return;
    }
    /// Queries all weapon holders
    type Query<'a> = (
        &'a Ability,            // Needed to test if active or not
        &'a Gun,                // Guns properties
        &'a mut Cooldown,       // Test and reset cooldown
        &'a mut Transform,      // Origin of bullets
    );
    let mut shots = Vec::new();
    for (_, (ability, gun, cooldown, transform)) in &mut world.query::<Query>() {
        // User input
        let Ok(input) = world.get::<&Input>(ability.owner) else {
            continue;
        };
        // Cooldown
        cooldown.0 -= time.dt();
        // Shooting
        if ability.active && cooldown.0 <= 0.0 && input.button(0) {
            shots.push((gun.shoot, ability.owner, transform.translation, input.look_axis()));
            *cooldown = gun.cooldown;
        }
    }
    for (shoot, e, o, v) in shots {
        (shoot)(world, e, o, v);
    }
}
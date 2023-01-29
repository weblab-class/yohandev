use hecs::{ Entity, EntityBuilder, World };
use nalgebra::Rotation2;

use crate::{
    transform::{ Transform, Parent, LocalPosition },
    math::vec2,
    render::{ Sprite, Costume }, input::{Input, FollowLookDirection}, bullet, platform::Time
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

/// Component that marks this entity as a shotgun ability
#[derive(Debug, Default)]
pub struct Shotgun {
    cooldown: f32,
}

pub fn shotgun_prefab(owner: Entity) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Shotgun::default(),
        Ability {
            owner,
            // TODO: ability switcher, this should be false.
            active: true,
        },
        Sprite::new(Costume::Shotgun {
            position: Default::default(),
            rotation: Default::default(),
        }),
        Transform::default(),
        Parent::new(owner),
        FollowLookDirection(owner),
        LocalPosition(vec2!(-15.0, 20.0)),
    ));
    builder
}

/// System that does the shotgun functionality
pub fn shotgun_controller(world: &mut World, time: &Time) {
    if cfg!(client) {
        return;
    }
    const N_BULLETS: usize = 20;    // how many bullets
    const SPREAD: f32 = 0.2;        // (+/-) radians
    const COOLDOWN: f32 = 1.5;      // seconds
    /// Queries all weapon holders
    type Query<'a> = (
        &'a Ability,            // Needed to test if active or not
        &'a mut Shotgun,        // Marker to query shotguns only
        &'a mut Transform,      // Origin of bullets
    );
    let mut shots = Vec::new();
    for (_, (ability, shotgun, transform)) in &mut world.query::<Query>() {
        // User input
        let Ok(input) = world.get::<&Input>(ability.owner) else {
            continue;
        };
        // Cooldown
        shotgun.cooldown -= time.dt();
        // Shooting
        if ability.active && shotgun.cooldown <= 0.0 && input.button(0) {
            shots.push((transform.translation, input.look_axis()));
            shotgun.cooldown = COOLDOWN;
        }
    }
    for (o, v) in shots {
        // TODO: this can be greatly optimized by simply sending the random seed
        for _ in 0..N_BULLETS {
            let v = Rotation2::new(SPREAD * (fastrand::f32() - 0.5)) * v;
            world.spawn(bullet::prefab(o, v).build());
        }
    }
}
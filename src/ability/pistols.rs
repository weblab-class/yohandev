use hecs::{ EntityBuilder, Entity };

use crate::{
    ability::{ Ability, Gun, Cooldown },
    transform::{ Transform, Parent, LocalPosition },
    render::{ Sprite, Costume },
    math::{ Rot2, vec2 },
    input::FollowLookDirection,
    health::Damage,
    bullet,
};

pub fn prefab(owner: Entity, binding: usize) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Ability {
            owner,
            binding,
            active: false,
        },
        Gun {
            // TODO these should come from `abilities.toml`
            cooldown: Cooldown(0.2),
            shoot: |world, owner, origin, velocity| {
                let spread = Rot2::new(0.01 * (fastrand::f32() - 0.5));
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
            },
        },
        Sprite::new(Costume::DualGun {
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
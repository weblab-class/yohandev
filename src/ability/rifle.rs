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

pub fn prefab(owner: Entity, binding: usize,) -> EntityBuilder {
    let mut builder = EntityBuilder::new();
    
    builder.add_bundle((
        Ability {
            owner,
            binding,
            active: false,
        },
        Gun {
            // TODO these should come from `abilities.toml`
            cooldown: Cooldown(0.07),
            shoot: |world, owner, origin, velocity| {
                let spread = Rot2::new(0.05 * (fastrand::f32() - 0.5));
                let velocity = 2000.0 * (spread * velocity);
                let damage = Damage {
                    amount: 2.0,
                    exclude: Some(owner),
                    destroy: true,
                };
                world.spawn(bullet::prefab(origin, velocity)
                    .add(damage)
                    .build()
                );
            },
        },
        Sprite::new(Costume::AssaultRifle {
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
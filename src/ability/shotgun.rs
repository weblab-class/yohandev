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

pub fn prefab(owner: Entity) -> EntityBuilder {
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
                    let spread = Rot2::new(0.1 * (fastrand::f32() - 0.5));
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
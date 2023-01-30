use hecs::{ Entity, World };

use crate::{
    ability::Ability,
    render::{ Sprite, Costume },
    transform::{ Transform, LocalPosition },
    input::FollowLookDirection,
    physics::Collider,
    math::{ Rot2, vec2 },
};

// Special parent with offsetted pivot
pub struct Shield(Entity);

pub fn instantiate(world: &mut World, owner: Entity, binding: usize) -> Entity {
    world.spawn((
        Ability {
            owner,
            binding,
            active: false,
        },
        Sprite::new(Costume::Shield {
            position: Default::default(),
            rotation: Default::default(),
        }),
        Shield(owner),
        Transform::default(),
        Collider::rect(25.0, 40.0),
        LocalPosition(vec2!(25.0, 0.0)),
        FollowLookDirection(owner),
    ))
}

/// System that positions the shield
pub fn position_shield(world: &mut World) {
    for (e, (transform, parent, position)) in &mut world.query::<(&mut Transform, &Shield, &LocalPosition)>() {
        if e != parent.0 {
            let Ok(target) = (unsafe {
                // SAFETY:
                // Asserted this entity isn't parented to itself
                world.get_unchecked::<&Transform>(parent.0)
            }) else {
                continue;
            };
            //      normally multiply by the parent's rotation     
            //                             special bit is here ----↴
            transform.translation = target.translation + Rot2::new(transform.rotation) * position.0;
        }
    }
}
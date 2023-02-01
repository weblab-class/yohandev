use hecs::World;

use crate::{
    math::{ Vec2, vec2 },
    physics::{ Collider, FixedBody },
    render::{ Sprite, Costume },
    transform::Transform, health::Health
};

fn platform(world: &mut World, pos: Vec2<f32>, width: f32) {
    world.spawn((
        Collider::rect(width, 20.0),
        FixedBody::default(),
        Sprite::new(Costume::Platform {
            position: pos,
            width
        }),
        Transform {
            translation: pos,
            rotation: 0.0,
        },
    ));
}

pub fn instantiate(world: &mut World) {
    platform(world, vec2!(125.0, 130.0), 275.0);
    platform(world, vec2!(500.0, 200.0), 300.0);
    platform(world, vec2!(400.0, 500.0), 500.0);
    platform(world, vec2!(800.0, 50.0), 400.0);
    platform(world, vec2!(1200.0, 350.0), 200.0);
    platform(world, vec2!(950.0, 250.0), 250.0);
    platform(world, vec2!(850.0, 400.0), 100.0);
}

/// System that instantly kills entities that fall off the map
pub fn void_damage(world: &mut World) {
    if cfg!(client) {
        return;
    }
    for (e, (health, transform)) in world.query_mut::<(&mut Health, &Transform)>() {
        if transform.translation.y < -1000.0 {
            health.now = 0.0;
            log::info!("{e:?} fell in the void.");
        }
    }
}
use hecs::Entity;
use parry2d::shape::{ Cuboid, Ball };

use crate::math::Vec2;

/// Collider component
pub enum Collider {
    Box(Cuboid),
    Circle(Ball),
}

/// Component that stores entities collided with in the past frame,
/// for entities that care about it. 
pub struct Collisions(Vec<Entity>);

/// Component for entities whose position is affected by its velocity
/// and collisions.
pub struct KinematicBody {
    velocity: Vec2<f32>,
}

/// Component for entities whose position is unaffected by collisions,
/// like a wall or the ground.
pub struct StaticBody;

/// Component denoting an entity as being affected by gravity.
pub struct Gravity {
    /// Acceleration of gravity in `m/s^2`(ie. `(0, -9.8)`)
    acceleration: Vec2<f32>,
}

/// Is this entity touching a static body?
pub enum Grounded {
    /// The entity is touching the ground and has been for `time`
    /// seconds.
    Yes { time: f32 },
    /// The entity is *not* touching the ground and has *not* been
    /// for `time` seconds. Useful for jump grace period.
    No { time: f32 },
}
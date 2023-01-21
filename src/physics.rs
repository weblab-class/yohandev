use std::ops::Deref;

use hecs::{ Entity, World, With };
use parry2d::{
    shape::{ Cuboid, Ball, Shape },
    query
};

use crate::{
    math::{ Vec2, vec2 },
    transform::Transform
};

/// Collider component
#[derive(Debug, Clone)]
pub enum Collider {
    Box(Cuboid),
    Circle(Ball),
}

impl Collider {
    /// Creates a new [Collider::Box]
    pub fn rect(w: f32, h: f32) -> Self {
        Self::Box(Cuboid::new(vec2!(w, h) / 2.0))
    }

    /// Creates a new [Collider::Circle]
    pub fn circle(r: f32) -> Self {
        Self::Circle(Ball::new(r))
    }
}

impl Deref for Collider {
    type Target = dyn Shape;

    fn deref(&self) -> &Self::Target {
        match self {
            Collider::Box(s) => s,
            Collider::Circle(s) => s,
        }
    }
}

/// Component that stores *all* entities collided with in the past frame,
/// for entities that care about it. 
#[derive(Debug, Default, Clone)]
pub struct Collisions(Vec<Entity>);

/// Component for entities whose position is affected by its velocity
/// and collisions.
/// 
/// [KinematicBody]'s react to [FixedBody]'s, but will not interact
/// with each other.
#[derive(Debug, Default, Clone)]
pub struct KinematicBody {
    /// Linear velocity.
    velocity: Vec2<f32>,
}

/// Component for entities whose position is unaffected by collisions,
/// like a wall or the ground.
#[derive(Debug, Default, Clone)]
pub struct FixedBody;

// TODO: DynamicBody for the fun destructible stuff

/// Component denoting an entity as being affected by gravity.
#[derive(Debug, Clone)]
pub struct Gravity {
    /// Acceleration of gravity in `m/s^2`(ie. `(0, -9.8)`)
    acceleration: Vec2<f32>,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            acceleration: vec2!(0.0, -9.8)
        }
    }
}

/// Is this entity touching a static body from below?
pub enum Grounded {
    /// The entity is touching the ground and has been for `time`
    /// seconds.
    Yes { time: f32 },
    /// The entity is *not* touching the ground and has *not* been
    /// for `time` seconds. Useful for jump grace period.
    No { time: f32 },
}

/// System that computes collisions between [Collider]s and stores the
/// result in [Collisions] components.
/// 
/// It should be called before other physics systems that might mutate
/// the [Collisions] component, as this one clears it.
pub fn compute_collisions(world: &mut World) {
    /// Minimum query to have a collision.
    type Query<'a> = (
        &'a Collider,
        &'a Transform,
    );
    // Simple O(n^2) `a` intersects `b` test:
    for (e1, ((c1, t1), Collisions(list))) in &mut world.query::<(Query, &mut Collisions)>() {
        // Reset collisions list
        list.clear();
        // Try every other collider...
        for (e2, (c2, t2)) in &mut world.query::<Query>() {
            // ...except self, of course:
            if e1 == e2 {
                continue;
            }
            if let Ok(intersects) = query::intersection_test(
                &t1.into(),
                &**c1,
                &t2.into(),
                &**c2,
            ) {
                // Collision!
                if intersects {
                    list.push(e2);
                }
            }
        }
    }
}

/// 
pub fn compute_grounded(world: &mut World) {
    todo!()
}

/// System that adds gravity to every relevant entity.
pub fn compute_gravity(world: &mut World) {
    /// Query kinematic bodies
    type Query<'a> = (
        &'a mut KinematicBody,
        &'a Gravity,
    );
    for (_, (kb, gravity)) in world.query_mut::<Query>() {
        kb.velocity += gravity.acceleration;
    }
}

/// System that steps the physics simulation.
/// TODO: use delta time
pub fn step_kinematic_bodies(world: &mut World) {
    /// Query the moving body.
    type KinematicQuery<'a> = (&'a KinematicBody, &'a Collider, &'a Transform);
    /// Query what it might bump into.
    type FixedQuery<'a> = With<(&'a Collider, &'a Transform), &'a FixedBody>;
    
    // Entities and their velocity * dt
    let mut moves = Vec::new();
    // Step every kinematic body by their velocity:
    for (e1, (kb, c1, t1)) in &mut world.query::<KinematicQuery>() {
        // Query the soonest impact
        let toi = world.query::<FixedQuery>()
            .into_iter()
            .filter(|(e2, _)| e1 != *e2)
            .map(|(_, (c2, t2))| {
                query::time_of_impact(
                    &t1.into(),
                    &kb.velocity,
                    &**c1,
                    &t2.into(),
                    &vec2!(0.0, 0.0),
                    &**c2,
                    1.0 / 60.0, // TODO:(important) delta time goes here
                    true,
                )
                .unwrap()
                .map(|toi| toi.toi)
                // If no collision occurs, move by the full velocity * dt
                .unwrap_or(1.0 / 60.0) // TODO: delta time goes here
            })
            .reduce(f32::min)
            .unwrap_or(1.0 / 60.0); // TODO: delta time goes here
        // Move
        moves.push((e1, kb.velocity * toi));
    }
    for (e, dpos) in moves {
        if let Ok(mut transform) = world.get::<&mut Transform>(e) {
            transform.translation += dpos;
        }
        // Nullify velocity's component along collision normal
        // proj formula
    }
}
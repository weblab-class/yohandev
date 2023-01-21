use std::ops::Deref;

use hecs::{ Entity, World, With };
use parry2d::{
    shape::{ Cuboid, Ball, Shape },
    query,
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
pub struct Collisions(pub Vec<Entity>);

/// Component for entities whose position is affected by its velocity
/// and collisions.
/// 
/// [KinematicBody]'s react to [FixedBody]'s, but will not interact
/// with each other.
#[derive(Debug, Default, Clone)]
pub struct KinematicBody {
    /// Linear velocity.
    pub velocity: Vec2<f32>,
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
    pub acceleration: Vec2<f32>,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            acceleration: vec2!(0.0, -981.0)
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

// /// System that computes collisions between [Collider]s and stores the
// /// result in [Collisions] components.
// /// 
// /// It should be called before other physics systems that might mutate
// /// the [Collisions] component, as this one clears it.
// pub fn compute_collisions(world: &mut World) {
//     /// Minimum query to have a collision.
//     type Query<'a> = (
//         &'a Collider,
//         &'a Transform,
//     );
//     // Simple O(n^2) `a` intersects `b` test:
//     for (e1, ((c1, t1), Collisions(list))) in &mut world.query::<(Query, &mut Collisions)>() {
//         // Reset collisions list
//         list.clear();
//         // Try every other collider...
//         for (e2, (c2, t2)) in &mut world.query::<Query>() {
//             // ...except self, of course:
//             if e1 == e2 {
//                 continue;
//             }
//             if let Ok(intersects) = query::intersection_test(
//                 &t1.into(),
//                 &**c1,
//                 &t2.into(),
//                 &**c2,
//             ) {
//                 // Collision!
//                 if intersects {
//                     list.push(e2);
//                 }
//             }
//         }
//     }
// }

/// System that updates the [Grounded] component.
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
        // TODO: delta time
        kb.velocity += gravity.acceleration * (1.0 / 60.0);
    }
}

/// System that simulates kinematic bodies.
pub fn compute_kinematics(world: &mut World) {
    type Query<'a> = (
        &'a mut Transform,
        &'a KinematicBody,
    );
    for (_, (transform, kb)) in world.query_mut::<Query>() {
        // TODO: delta time
        transform.translation += kb.velocity * (1.0 / 60.0);
    }
}

/// System that resolves intersections between kinematic/fixed bodies.
pub fn resolve_collisions(world: &mut World) {
    /// Minimum query to have a collision.
    type KinematicQuery<'a> = (
        &'a mut Transform,
        &'a mut KinematicBody,
        &'a Collider,
    );
    type FixedQuery<'a> = With<(&'a Transform, &'a Collider), &'a FixedBody>;

    // Simple O(n^2) `a` intersects `b` test.
    for (_, (t1, kb, c1)) in &mut world.query::<KinematicQuery>() {
        for (_, (t2, c2)) in &mut world.query::<FixedQuery>() {
            if let Ok(Some(contact)) = query::contact(
                &(&*t1).into(),
                &**c1,
                &t2.into(),
                &**c2,
                0.01,
            ) {
                if contact.dist >= 0.0 {
                    continue;
                }
                let norm = contact.normal1.into_inner();
                // Remove component of translation along contact normal
                t1.translation += norm * contact.dist;
                // Remove component of velocity along contact normal
                kb.velocity -= norm * norm.dot(&kb.velocity);
            }
        }
    }
}
use std::ops::Deref;

use hecs::{ Entity, World, With };
use parry2d::{
    shape::{ Cuboid, Ball, Shape },
    query::{ self, Contact, Ray },
};
use smallvec::SmallVec;

use crate::{
    math::{ Vec2, vec2 },
    transform::Transform,
    platform::Time, ability::TimeScale,
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
pub struct Collisions(pub SmallVec<[Entity; 8]>);

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

/// Is this entity touching a "ground" static body?
/// Definition of ground here relates to this entity's [Gravity], ie.
/// this component is essentially useless without it.
#[derive(Debug, Clone)]
pub enum Grounded {
    /// The entity is touching the ground and has been for `time` seconds
    /// seconds.
    Yes { time: f32 },
    /// The entity is *not* touching the ground and has *not* been
    /// for `time` seconds. Useful for jump grace period.
    No { time: f32 },
}

impl Default for Grounded {
    fn default() -> Self {
        Self::No { time: 0.0 }
    }
}

/// System that adds gravity to every relevant entity.
pub fn compute_gravity(world: &mut World, time: &Time) {
    /// Query kinematic bodies
    type Query<'a> = (
        &'a mut KinematicBody,
        &'a Gravity,
        Option<&'a TimeScale>,
    );
    for (_, (kb, gravity, scale)) in world.query_mut::<Query>() {
        let scale = scale
            .map(|s| s.0)
            .unwrap_or(1.0);
        kb.velocity += gravity.acceleration * time.dt() * scale;
    }
}

/// System that simulates kinematic bodies.
pub fn compute_kinematics(world: &mut World, time: &Time) {
    type Query<'a> = (
        &'a mut Transform,
        &'a KinematicBody,
        Option<&'a TimeScale>,
    );
    for (_, (transform, kb, scale)) in world.query_mut::<Query>() {
        let scale = scale
            .map(|s| s.0)
            .unwrap_or(1.0);
        transform.translation += kb.velocity * time.dt() * scale;
    }
}

/// System that resolves intersections between kinematic/fixed bodies.
/// Also updates the [Grounded] component.
pub fn resolve_collisions(world: &mut World, time: &Time) {
    /// Minimum query to have a collision.
    type KinematicQuery<'a> = (
        &'a mut Transform,
        &'a mut KinematicBody,
        &'a Collider,
        Option<(&'a mut Grounded, &'a Gravity)>,
        Option<&'a TimeScale>,
    );
    type FixedQuery<'a> = With<(&'a Transform, &'a Collider), &'a FixedBody>;

    // Simple O(n^2) `a` intersects `b` test.
    for (_, (t1, kb, c1, mut ground, scale)) in &mut world.query::<KinematicQuery>() {
        // Cache the body's gravity for groundedness computations.
        let gravity = match ground {
            Some((_, g)) => g.acceleration.normalize(),
            _ => vec2!(0.0, 0.0)
        };
        let scale = scale
            .map(|s| s.0)
            .unwrap_or(1.0);
        // Find at least one "ground"
        let mut grounded = false;

        for (_, (t2, c2)) in &mut world.query::<FixedQuery>() {
            // Compute collision:
            let Ok(contact) = query::contact(
                &(&*t1).into(),
                c1.deref(),
                &t2.into(),
                c2.deref(),
                0.01,
            ) else {
                continue;
            };
            // Compute the contact normal and correct overlaps.
            let Some(Contact { dist, normal1, .. }) = contact else {
                continue;
            };

            if dist <= 0.0 {
                let n = normal1.into_inner();
                
                // Remove component of translation along contact normal.
                t1.translation += n * dist;
                // Remove component of velocity along contact normal.
                kb.velocity -= n * n.dot(&kb.velocity);
                // Compute groundedness
                grounded |= n.dot(&gravity.normalize()) > 0.5;
            }
        }
        // (Optionally) compute groundedness
        if let Some((g, _)) = &mut ground {
            **g = match g.clone() {
                Grounded::Yes { time: t } if grounded => Grounded::Yes {
                    time: t + time.dt() * scale,
                },
                Grounded::No { time: t } if !grounded => Grounded::No {
                    time: t + time.dt() * scale,
                },
                Grounded::No { .. } => Grounded::Yes { time: 0.0 },
                Grounded::Yes { .. } => Grounded::No { time: 0.0 },
            };
        }
    }
}

/// System that computes collisions and stores them in [Collisions]
pub fn compute_collisions(world: &mut World) {
    for (e1, (t1, c1, collisions)) in &mut world.query::<(&Transform, &Collider, &mut Collisions)>() {
        collisions.0.clear();
        for (e2, (t2, c2)) in &mut world.query::<(&Transform, &Collider)>() {
            // Don't collide with self!
            if e1 == e2 {
                continue;
            }
            // Compute collision:
            let Ok(contact) = query::intersection_test(
                &(&*t1).into(),
                c1.deref(),
                &t2.into(),
                c2.deref(),
            ) else {
                continue;
            };
            if contact {
                collisions.0.push(e2);
            }
        }
    }
}

/// Utility function to send a raycast in the scene and get the entity
/// hit and at what position
pub fn raycast(
    world: &World,
    origin: Vec2<f32>,
    dir: Vec2<f32>,
    ignore: Option<Entity>,
) -> Option<(Entity, Vec2<f32>)> {
    let Some(dir) = dir.try_normalize(0.001) else {
        return None;
    };
    let ray = Ray::new(origin.into(), dir);
    // Find min TOI
    world.query::<(&Transform, &Collider)>()
        .iter()
        .filter_map(|(e, (transform, collider))| {
            let toi = collider.cast_ray(
                &(&*transform).into(),
                &ray,
                std::f32::MAX,
                true
            )?;
            Some((e, toi))
        })
        .filter(|(e, _)| ignore != Some(*e))
        .min_by(|(_, x), (_, y)|
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        )
        .map(|(entity, toi)| (
            entity,
            origin + dir * toi,
        ))
}

/// Utility function to send a raycast in the scene and get the entity
/// hit and at what position
pub fn raycast_solid(
    world: &World,
    origin: Vec2<f32>,
    dir: Vec2<f32>,
    ignore: Option<Entity>,
) -> Option<(Entity, Vec2<f32>)> {
    let Some(dir) = dir.try_normalize(0.001) else {
        return None;
    };
    let ray = Ray::new(origin.into(), dir);
    // Find min TOI
    world.query::<With<(&Transform, &Collider), &FixedBody>>()
        .iter()
        .filter_map(|(e, (transform, collider))| {
            let toi = collider.cast_ray(
                &(&*transform).into(),
                &ray,
                std::f32::MAX,
                true
            )?;
            Some((e, toi))
        })
        .filter(|(e, _)| ignore != Some(*e))
        .min_by(|(_, x), (_, y)|
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        )
        .map(|(entity, toi)| (
            entity,
            origin + dir * toi,
        ))
}
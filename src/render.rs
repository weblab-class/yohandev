use hecs::World;

use crate::{
    platform::Canvas,
    transform::{Transform, Parent},
    math::Vec2, ability::Ability, health::Health
};

/// A type of [Sprite]
/// 
/// SAFETY:
/// This type is passed directly to `platform/`.
/// For Typescript binding simplicity, every field should be
/// aligned to 4 bytes(ie. `u32`, `f32`).
#[derive(Debug)]
#[repr(u32)]
pub enum Costume {
    Player {
        position: Vec2<f32>,
        scale: Vec2<f32>,
        lean: f32,
    },
    Bullet {
        position: Vec2<f32>,
    },
    Shotgun {
        position: Vec2<f32>,
        rotation: f32,
    },
    HealthBar {
        position: Vec2<f32>,
        percentage: f32,
    },
    AssaultRifle {
        position: Vec2<f32>,
        rotation: f32,
    }
}

/// Whether a [Sprite] is visible or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Visibility {
    /// Sprite is shown.
    #[default]
    Shown,
    /// Sprite is not shown, but still exists in DOM.
    Hidden,
}

/// Component for entities with a 2D costume
#[derive(Debug)]
pub struct Sprite {
    /// Type of sprite. It cannot changed after initialization
    pub costume: Costume,
    /// Whether the sprite is visible.
    pub visibility: Visibility,
    /// Handle of the `platform`'s object(for drop management).
    pub handle: Option<u32>,
}

impl Sprite {
    /// Create a new sprite component
    pub fn new(costume: Costume) -> Self {
        Self {
            costume,
            visibility: Default::default(),
            handle: None,
        }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        Canvas::remove(self);
    }
}

/// System that animates player sprites' squash/stretch
pub fn animate_player_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, sprite)) in world.query_mut::<(&Transform, &mut Sprite)>() {
        let Costume::Player { position, scale, lean } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        let delta = target - *position;
        
        *position = target;
        // Lean in direction of movement unless jumping/falling
        *lean = delta.x;
        // Squash/stretch
        scale.x = 1.0 - 0.01 * delta.y.abs();
        scale.y = 1.0 + 0.02 * delta.y.abs();
    }
}

/// System that animates bullets
pub fn animate_bullet_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, sprite)) in world.query_mut::<(&Transform, &mut Sprite)>() {
        let Costume::Bullet { position } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        // TODO: trail
        let _delta = target - *position;

        *position = target;
    }
}

pub fn animate_basic_gun_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, ability, sprite)) in world.query_mut::<(&Transform, &Ability, &mut Sprite)>() {
        let (position, rotation) = match &mut sprite.costume {
            Costume::Shotgun { position, rotation } => (position, rotation),
            Costume::AssaultRifle { position, rotation } => (position, rotation),
            _ => {
                continue;
            }
        };
        let target = transform.translation;
        let delta = target - *position;

        // Damp position
        *position += 0.9 * delta;
        // Rotation is exact
        *rotation = transform.rotation;
        // Visibility
        sprite.visibility = match ability.active {
            true => Visibility::Shown,
            false => Visibility::Hidden,
        };
    }
}

pub fn animate_health_bar_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, parent, sprite)) in &mut world.query::<(&Transform, &Parent, &mut Sprite)>() {
        let Costume::HealthBar { position, percentage } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        let delta = target - *position;

        // Damp position
        *position += 0.9 * delta;
        // Percentage
        if let Ok(health) = world.get::<&Health>(parent.0) {
            *percentage = health.now / health.max;
        }
    }
}

/// System that draws sprites
pub fn draw_sprites(world: &mut World, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (_, sprite) in world.query_mut::<&mut Sprite>() {
        canvas.draw(sprite);
    }
}
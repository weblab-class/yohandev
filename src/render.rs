use hecs::World;

use crate::{
    platform::Canvas,
    transform::Transform,
    math::Vec2
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
    }
}

/// Component for entities with a 2D costume
#[derive(Debug)]
pub struct Sprite {
    /// Type of sprite. It cannot changed after initialization
    pub costume: Costume,
    /// Handle of the `platform`'s object(for drop management).
    pub handle: Option<u32>,
}

impl Sprite {
    /// Create a new sprite component
    pub fn new(costume: Costume) -> Self {
        Self { costume, handle: None }
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
    for (e, (transform, sprite)) in world.query_mut::<(&Transform, &mut Sprite)>() {
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
    for (e, (transform, sprite)) in world.query_mut::<(&Transform, &mut Sprite)>() {
        let Costume::Bullet { position } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        // TODO: trail
        let _delta = target - *position;

        *position = target;
    }
}

/// System that draws sprites
pub fn draw_sprites(world: &mut World, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (e, sprite) in world.query_mut::<&mut Sprite>() {
        canvas.draw(sprite);
    }
}
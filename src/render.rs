use hecs::{ World, Entity };

use crate::{
    platform::Canvas,
    transform::{ Transform, Parent },
    math::Vec2
};

/// Component that marks this entity as having a player sprite.
#[derive(Debug, Default)]
pub struct PlayerSprite {
    /// Owner so it can be removed from DOM when dropped(temporary)
    handle: Option<Entity>,
    /// Last position sent to render
    last_position: Option<Vec2<f32>>,
    /// X-axis skew
    lean: f32,
    /// Squash/stretch
    scale: Vec2<f32>,
}

impl Drop for PlayerSprite {
    fn drop(&mut self) {
        if let Some(entity) = self.handle {
            Canvas::remove(entity.id());
        }
    }
}

/// System that animates player sprites' squash/stretch
pub fn animate_player_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (e, (transform, sprite)) in world.query_mut::<(&Transform, &mut PlayerSprite)>() {
        let pos = transform.translation;
        let vel = pos - sprite.last_position.unwrap_or(pos);
        
        sprite.handle = Some(e);
        sprite.last_position = Some(pos);
        // Lean in direction of movement unless jumping/falling
        sprite.lean = vel.x;// * (1.0 / (1.0 + 0.3 * vel.y.abs()));
        // Squash/stretch
        sprite.scale.x = 1.0 - 0.01 * vel.y.abs();
        sprite.scale.y = 1.0 + 0.02 * vel.y.abs();
    }
}

/// System that draws the player sprites
pub fn draw_player_sprites(world: &mut World, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (e, (transform, sprite)) in world.query_mut::<(&Transform, &PlayerSprite)>() {
        canvas.draw_player(
            e.id(),
            transform.translation.x,
            transform.translation.y,
            sprite.lean,
            sprite.scale.x,
            sprite.scale.y,
        );
    }
}

/// Component to give a bullet appearance
#[derive(Debug, Default)]
pub struct BulletSprite {
    /// Owner so it can be removed from DOM when dropped(temporary)
    handle: Option<Entity>,   
}

impl Drop for BulletSprite {
    fn drop(&mut self) {
        if let Some(entity) = self.handle {
            Canvas::remove(entity.id());
        }
    }
}

/// System that draws bullets in the scene
pub fn draw_bullet_sprites(world: &mut World, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (e, (transform, sprite)) in world.query_mut::<(&Transform, &mut BulletSprite)>() {
        sprite.handle = Some(e);
        canvas.draw_bullet(
            e.id(),
            transform.translation.x,
            transform.translation.y,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum HandheldSpriteKind {
    Shotgun,
    AssaultRifle,
    DualGun,
    Shield,
}

/// Component to give a handheld(gun, shield, etc.) appearance
#[derive(Debug)]
pub struct HandheldSprite {
    pub kind: HandheldSpriteKind,
    /// Owner so it can be removed from DOM when dropped(temporary)
    handle: Option<Entity>,
}

impl HandheldSprite {
    pub fn new(kind: HandheldSpriteKind) -> Self {
        Self { kind, handle: None }
    }
}

impl Drop for HandheldSprite {
    fn drop(&mut self) {
        if let Some(entity) = self.handle {
            Canvas::remove(entity.id());
        }
    }
}

/// System the draws handheld sprites
pub fn draw_handheld_sprites(world: &mut World, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (e, (transform, sprite)) in world.query_mut::<(&Transform, &mut HandheldSprite)>() {
        sprite.handle = Some(e);
        canvas.draw_handheld(
            e.id(),
            sprite.kind,
            transform.translation.x,
            transform.translation.y,
        );
    }
}
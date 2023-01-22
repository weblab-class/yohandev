use hecs::{ World, Entity };

use crate::{
    platform::{Canvas, Time},
    transform::Transform,
    math::Vec2, vec2,
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
pub fn animate_player_sprites(world: &mut World, time: &Time) {
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
        // Bob up/down
        // sprite.scale.y += 0.05 * (3.0 * time.elapsed()).cos();
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
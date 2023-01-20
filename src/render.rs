use hecs::World;

use crate::{ platform::Canvas, transform::Transform };

/// Component that gives entities a visual appearance.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Sprite {
    Rect,
    Circle,
}

/// `Client System`:
///     - Updates sprites
/// ` Server System`:
///     - Does nothing
pub fn update(world: &World, canvas: &Canvas) {
    if cfg!(server) {
        // Nothing to do :O
        return;
    }
    for (e, (sprite, transform)) in &mut world.query::<(&Sprite, &Transform)>() {
        canvas.set(
            e.id(),
            *sprite,
            transform.translation.x,
            transform.translation.y,
        );
    }
}
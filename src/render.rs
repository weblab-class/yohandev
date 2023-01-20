use hecs::World;

use crate::{ platform::Canvas, input::Input };

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
    // TODO: use transform component
    for (e, (sprite, input)) in &mut world.query::<(&Sprite, &Input)>() {
        canvas.set(
            e.id(),
            *sprite,
            100.0 + input.dx() * 50.0,
            100.0 + input.dy() * 50.0,
        );
    }
}
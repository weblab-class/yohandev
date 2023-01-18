pub use bevy_ecs::prelude as ecs;
pub use vek as math;

use once_cell::unsync::Lazy;
use ecs::{ World, Component };

mod net;
mod draw;

/// Behold, the quick & shitty™ solution to not having an
/// even system. WASM is not threaded, so it's just easier
/// to have a global state and hard-code the event loop.
static mut WORLD: Lazy<World> = Lazy::new(|| World::new());

#[no_mangle]
pub extern "C" fn setup() {
    // SAFETY:
    // WebAssembly is single threaded.
    let world = unsafe { &mut *WORLD };
}

/// Step the game state by one tick.
#[no_mangle]
pub extern "C" fn tick(dt: u32) {
    // SAFETY:
    // WebAssembly is single threaded.
    let world = unsafe { &mut *WORLD };

    
}
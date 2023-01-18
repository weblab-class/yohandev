use hecs::World;
use once_cell::unsync::Lazy;

mod net;
mod draw;

pub fn world<'a>() -> &'a mut World {
    // Global instance since WebAssembly must yield control back
    // to JS and rely on its callbacks.
    static mut WORLD: Lazy<World> = Lazy::new(|| World::new());

    unsafe {
        // SAFETY:
        // WebAssembly is single threaded, this is probably fine.
        &mut *WORLD
    }
}

#[no_mangle]
pub extern "C" fn main() {
    let world = world();

    // Crappy event system:
    draw::spawn_sprites(world, 1000);
}

#[no_mangle]
pub extern "C" fn tick(_time: u32) {
    let world = world();
    
    // Crappy event system:
    draw::render(world);
    draw::wiggle(world);
}
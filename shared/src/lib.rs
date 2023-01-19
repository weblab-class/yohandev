pub use packets::Packet;

mod packets;
mod log;

pub mod ecs {
    use once_cell::unsync::Lazy;
    // Re-export `hecs`
    pub use hecs::*;

    /// Get the global instance of the world.
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
}
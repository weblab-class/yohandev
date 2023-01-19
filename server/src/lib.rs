use shared::ecs;

mod net;

#[no_mangle]
pub extern "C" fn main() {
    let world = ecs::world();

    // Crappy event system:
}

/// Step the game state by one tick.
#[no_mangle]
pub extern "C" fn tick() {
    let world = ecs::world();
 
    // Crappy event system:
    for (id, packet) in net::poll() {
        
    }
}
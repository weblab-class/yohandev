mod net;

pub extern "C" fn main() {
    
}

/// Step the game state by one tick.
#[no_mangle]
pub extern "C" fn tick() {
    // Ping <-> Pong
    for (id, packet) in net::poll() {
        
    }
}
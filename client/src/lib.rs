use once_cell::unsync::Lazy;
use shared::Packet;
use hecs::World;
use net::Network;

mod net;

/// Behold, the quick & shitty™ solution to not having an
/// even system. WASM is not threaded, so it's just easier
/// to have a global state and hard-code the event loop.
static mut WORLD: Lazy<World> = Lazy::new(|| World::new());

#[no_mangle]
pub extern "C" fn setup() {
    // SAFETY:
    // WebAssembly is single threaded.
    let world = unsafe { &mut *WORLD };
    
    world.spawn((0.0,));
}

/// Step the game state by one tick.
#[no_mangle]
pub extern "C" fn tick() {
    // SAFETY:
    // WebAssembly is single threaded.
    let world = unsafe { &mut *WORLD };

    // Ping <-> Pong
    for packet in Network::poll() {
        log::info!("Got {packet:?} from the server!");
    }
    Network::send(&Packet::Ping);

    world.spawn((0.0,));
}
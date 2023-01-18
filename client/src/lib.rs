use shared::Packet;
use net::Network;

mod net;

/// Step the game state by one tick.
#[no_mangle]
pub extern "C" fn tick() {
    // Ping <-> Pong
    for packet in Network::poll() {
        log::info!("Got {packet:?} from the server!");
    }
    Network::send(&Packet::Ping);
}
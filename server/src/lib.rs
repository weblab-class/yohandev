use shared::Packet;
use net::Network;

mod net;

/// Step the game state by one tick.
#[no_mangle]
pub extern "C" fn tick() {
    // Ping <-> Pong
    for (id, packet) in Network::poll() {
        Network::send(id, &match packet {
            Packet::Ping => Packet::Pong,
            Packet::Pong => Packet::Ping,
        });
        log::info!("Got {packet:?} from {id:?}");
    }
}
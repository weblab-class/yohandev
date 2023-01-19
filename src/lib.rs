mod platform;
mod packets;
mod render;
mod input;

pub fn setup() {
    log::info!("Program has started!");
    
    if cfg!(server) {
        log::info!("I am the server!");
    } else {
        log::info!("I am the client!");
    }
}

pub fn tick() {
    // TODO instantiate once, then get form platform.
    let mut socket = platform::Socket::default();
    let input = platform::Gamepad::default();

    socket.poll();
    for (from, packet) in socket.packets() {
        log::info!("Received {packet:?} from {from:?}");
    }
    for connection in socket.connections() {
        socket.send(*connection, &packets::Packet::Ping);
    }

    if cfg!(client) {
        log::info!("x: {}, y: {}", input.dx(), input.dy());
    }
}
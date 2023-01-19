use shared::ecs;

mod net;
mod draw;
mod input;

#[no_mangle]
pub extern "C" fn main() {
    
}

#[no_mangle]
pub extern "C" fn tick(_time: u32) {
    let world = ecs::world();
    
    // Input
    input::poll();
    // Network
    for packet in net::poll() {
        net::spawn::players(world, &packet);
    }
    // Gameplay
    // -- snip --
    if input::is_down(input::Key::Up) {
        log::info!("UP!");
    }
    // Render
    draw::render(world);
}
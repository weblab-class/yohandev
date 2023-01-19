mod platform;
mod packets;
mod render;
mod input;

pub fn setup() {
    log::info!("Program has started!");
}

pub fn tick() {
    log::info!("Tick, tock, tick, tock...");
}
use platform::{ Canvas, Gamepad, Socket };
use hecs::World;

mod transform;
mod platform;
mod packets;
mod player;
mod render;
mod input;

pub fn main() {
    let mut world = World::new();
    let mut socket = Socket::default();
    let canvas = Canvas::default();
    let input = Gamepad::default();

    platform::run(move || {
        socket.poll();

        player::spawn_existing(&mut world, &socket);
        player::spawn(&mut world, &socket);
        input::update(&mut world, &input);
        player::controller(&mut world);
        render::update(&world, &canvas);
    });
}
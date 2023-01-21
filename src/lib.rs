use platform::{ Canvas, Gamepad, Socket };
use hecs::World;

mod transform;
mod platform;
mod network;
mod physics;
mod player;
mod spawn;
mod render;
mod input;
mod math;

pub fn main() {
    let mut world = World::new();
    let mut socket = Socket::default();
    let canvas = Canvas::default();
    let input = Gamepad::default();

    platform::run(move || {
        socket.poll();

        player::instantiate(&mut world, &socket);
        spawn::networked_instantiate(&mut world, &socket);
        input::update(&mut world, &input);
        input::network_player_commands(&mut world, &socket);
        physics::compute_collisions(&mut world);
        player::controller(&mut world);
        transform::networked_position(&mut world, &socket);
        render::update(&world, &canvas);
    });
}
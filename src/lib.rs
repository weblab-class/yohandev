use platform::{ Canvas, Gamepad, Socket };
use hecs::World;

mod platform;
mod packets;
mod render;
mod input;

pub fn main() {
    let mut world = World::new();
    let mut socket = Socket::default();
    let canvas = Canvas::default();
    let input = Gamepad::default();

    world.spawn((
        input::Input::default(),
        render::Sprite::Rect,
    ));

    platform::run(move || {
        input::update(&mut world, &input);
        render::update(&world, &canvas);
    });
}
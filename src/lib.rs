use platform::{ Canvas, Gamepad, Socket, Time };
use hecs::World;

mod transform;
mod platform;
mod network;
mod physics;
mod render;
mod bullet;
mod player;
mod input;
mod math;

pub fn main() {
    let mut world = World::new();
    let mut socket = Socket::default();
    let mut time = Time::default();
    let canvas = Canvas::default();
    let input = Gamepad::default();

    // Test game level
    world.spawn((
        render::Sprite::Rect,
        // collider is bigger than visual sprite
        physics::Collider::rect(500.0, 20.0),
        physics::FixedBody::default(),
        transform::Transform::default(),
    ));
    world.spawn((
        render::Sprite::Rect,
        // collider is bigger than visual sprite
        physics::Collider::rect(20.0, 200.0),
        physics::FixedBody::default(),
        transform::Transform::default(),
    ));

    platform::run(move || {
        socket.poll();
        time.poll();

        player::networked_instantiate(&mut world, &socket);
        input::update(&mut world, &input);
        input::network_player_commands(&mut world, &socket);
        // TODO: client-side prediction
        if cfg!(server) {
            player::controller(&mut world, &time);
            physics::compute_gravity(&mut world, &time);
            physics::compute_kinematics(&mut world, &time);
            physics::resolve_collisions(&mut world, &time);
        }
        transform::networked_position(&mut world, &socket);
        render::update(&world, &canvas);
    });
}
use platform::{ Canvas, Gamepad, Socket, Time };
use hecs::World;

mod transform;
mod platform;
mod network;
mod physics;
mod ability;
mod render;
mod bullet;
mod player;
mod health;
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
        physics::Collider::rect(5000.0, 20.0),
        physics::FixedBody::default(),
        transform::Transform {
            translation: vec2!(0.0, 100.0),
            rotation: 0.0,
        },
    ));
    world.spawn((
        physics::Collider::rect(20.0, 300.0),
        physics::FixedBody::default(),
        transform::Transform::default(),
    ));

    platform::run(move || {
        socket.poll();
        time.poll();

        player::networked_instantiate(&mut world, &socket);
        player::networked_despawn(&mut world, &socket);
        input::update(&mut world, &input);
        input::network_player_commands(&mut world, &socket);
        // TODO: client-side prediction
        if cfg!(server) {
            player::platformer_controller(&mut world, &time);
        }
        transform::local_to_world(&mut world);
        physics::compute_gravity(&mut world, &time);
        physics::compute_kinematics(&mut world, &time);
        physics::resolve_collisions(&mut world, &time);
        physics::compute_collisions(&mut world);
        transform::networked_position(&mut world, &socket);
        ability::shotgun_controller(&mut world, &time);
        input::network_look_direction(&mut world, &socket);
        input::follow_look_direction(&mut world);
        health::deal_damage(&mut world, &socket);
        bullet::network_instantiate(&mut world, &socket);
        bullet::despawn_bullets(&mut world, &time);
        render::animate_player_sprites(&mut world);
        render::animate_bullet_sprites(&mut world);
        render::animate_shotgun_sprites(&mut world);
        render::animate_health_bar_sprites(&mut world);
        render::draw_sprites(&mut world, &canvas);
    });
}
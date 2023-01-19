use draw::Sprite;
use net::player::OwnedPlayer;
use shared::{ ecs, Transform, Player, Input };

mod net;
mod draw;
mod input;

#[no_mangle]
pub extern "C" fn main() {
    let world = ecs::world();

    // TODO: spawn via network
    world.spawn((
        Transform::default(),
        Player,
        OwnedPlayer,
        Sprite::Circle(Default::default(), 20.0),
        Input::default(),
    ));
}

#[no_mangle]
pub extern "C" fn tick(_time: u32) {
    let world = ecs::world();
    
    // Input
    input::poll(world);
    // Network
    for packet in net::poll() {
        net::player::spawn(world, &packet);
    }
    net::player::input(world);
    // Gameplay
    shared::player::move_players(world);
    // Render
    draw::render(world);
}
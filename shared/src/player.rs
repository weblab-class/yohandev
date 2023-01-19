use vek::Vec2;

use crate::{
    ecs::{ World, With },
    Transform,
    Input,
};

/// Component that marks an entity as a player.
pub struct Player;

/// System that moves players according to their input.
pub fn move_players(world: &mut World) {
    type Query<'a> = With<(&'a mut Transform, &'a Input), &'a Player>;

    for (_, (transform, input)) in world.query_mut::<Query>() {
        // TODO: delta time
        transform.translation += Vec2 {
            x: input.dx as f32 / 100.0,
            y: input.dy as f32 / 100.0,
        };
    }
}
use shared::{ ecs::World, Input };

// -----------------------[ FFI ]-----------------------
extern {
    fn poll_input_dx() -> i8;
    fn poll_input_dy() -> i8;
}
// -----------------------------------------------------

/// System that polls buffered inputs each frame and updates
/// input components accordingly.
pub fn poll(world: &mut World) {
    let dx = unsafe {
        // SAFETY:
        // Just a normal function call.
        poll_input_dx()
    };
    let dy = unsafe {
        // SAFETY:
        // Just a normal function call.
        poll_input_dy()
    };

    // Update entities
    for (_, input) in world.query_mut::<&mut Input>() {
        input.dx = dx;
        input.dy = dy;
    }
}
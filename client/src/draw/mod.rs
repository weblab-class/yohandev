use crate::ecs::{ Entity, Component, Query, Changed };
use crate::math::{ Vec2, Extent2 };

// -----------------------[ FFI ]-----------------------
extern {
    // SAFETY:
    // Discriminant in `Shape` is explicitely marked `u32`
    // to guarentee a 4 byte alignment.
    fn query_shapes(idx: u32, shape: *const Shape);
}
// -----------------------------------------------------

/// [Component] that draws a 2D shape.
#[derive(Component, Debug, Clone)]
#[repr(u32)]
pub enum Shape {
    /// Rectangle with `(pos, size)` relative to entity's position.
    Rect(Vec2<f32>, Extent2<f32>),
    /// Quad with `vertices[4]` relative to enttiy's position.
    Quad([Vec2<f32>; 4]),
    /// Circle with `(pos, radius)` relative to entity's position.
    Circle(Vec2<f32>, f32),
}

/// System that updates all [Shape]s in the world.
pub fn render(query: Query<(Entity, &Shape), Changed<Shape>>) {
    for (ent, shape) in &query {
        unsafe {
            // SAFETY:
            // Borrow of `shape` lasts for the duration of the function
            // call.
            query_shapes(ent.index(), shape);
        }
    }
}
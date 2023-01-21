pub use nalgebra::Vector2 as Vec2;

pub use crate::vec2;

/// Short-hand for creating a 2D vector.
#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        nalgebra::Vector2::new($x, $y)
    };
}
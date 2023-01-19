use vek::{ Vec2, Extent2 };
use shared::ecs::World;

// -----------------------[ FFI ]-----------------------
extern {
    fn iter_sprites_rect(e: u32, x: f32, y: f32, w: f32, h: f32);
    fn iter_sprites_circle(e: u32, x: f32, y: f32, r: f32);
}
// -----------------------------------------------------

/// Component to display a sprite. All coordinates are
/// TODO: relative to one's [Pos], if any.
pub enum Sprite {
    Rect(Vec2<f32>, Extent2<f32>),
    Circle(Vec2<f32>, f32),
}

/// System that updates all [Sprite]s.
pub fn render(world: &mut World) {
    for (entity, shape) in &mut world.query::<&Sprite>() {
        match shape {
            Sprite::Rect(Vec2 { x, y }, Extent2 { w, h }) => unsafe {
                iter_sprites_rect(entity.id(), *x, *y, *w, *h);
            },
            Sprite::Circle(Vec2 { x, y }, r) => unsafe {
                iter_sprites_circle(entity.id(), *x, *y, *r);
            },
        }
    }
}
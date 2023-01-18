use vek::{ Vec2, Extent2 };
use hecs::World;

// -----------------------[ FFI ]-----------------------
extern {
    fn iter_sprites_rect(e: u32, x: f32, y: f32, w: f32, h: f32);
    fn iter_sprites_circle(e: u32, x: f32, y: f32, r: f32);
    fn rand() -> f32;
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

/// Test system to spawn lots of [Sprite]s
pub fn spawn_sprites(world: &mut World, n: usize) {
    fn random() -> f32 {
        unsafe { rand() }
    }

    for _ in 0..n {
        world.spawn((if random() > 0.5 {
            Sprite::Circle(
                Vec2::new(random() * 1000.0, random() * 1000.0),
                random() * 10.0
            )
        } else {
            Sprite::Rect(
                Vec2::new(random() * 1000.0, random() * 1000.0),
                Extent2::new(random() * 10.0, random() * 10.0),
            )
        },));
    }
}

pub fn wiggle(world: &mut World) {
    use Sprite::*;

    // Get a random vector.
    fn rvec() -> Vec2<f32> {
        Vec2 {
            x: unsafe { rand() - 0.5 },
            y: unsafe { rand() - 0.5 },
        }
    }

    for (_, shape) in world.query_mut::<&mut Sprite>() {
        match shape {
            Rect(pos, _) | Circle(pos, _ )=> {
                *pos += rvec();
            }
        }
    }
}
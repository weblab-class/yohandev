use hecs::{World, Entity};

use crate::{
    platform::{Canvas, Socket},
    transform::{Transform, Parent},
    math::{ Vec2, vec2 },
    ability::{Ability, BubbleShield, Cooldown},
    health::Health, physics, network::Packet
};

/// A type of [Sprite]
/// 
/// SAFETY:
/// This type is passed directly to `platform/`.
/// For Typescript binding simplicity, every field should be
/// aligned to 4 bytes(ie. `u32`, `f32`).
#[derive(Debug, Clone)]
#[repr(u32)]
pub enum Costume {
    Player {
        position: Vec2<f32>,
        scale: Vec2<f32>,
        lean: f32,
        color: i32,
    },
    Bullet {
        position: Vec2<f32>,
    },
    Shotgun {
        position: Vec2<f32>,
        rotation: f32,
    },
    HealthBar {
        position: Vec2<f32>,
        percentage: f32,
    },
    AssaultRifle {
        position: Vec2<f32>,
        rotation: f32,
    },
    DualGun {
        position: Vec2<f32>,
        rotation: f32,
    },
    Shield {
        position: Vec2<f32>,
        rotation: f32,
    },
    Push {
        position: Vec2<f32>,
    },
    Freeze,
    Lightning {
        /// Position of impact
        position: Vec2<f32>,
    },
    BubbleShield {
        position: Vec2<f32>,
        radius: f32,
    },
    Heal {
        position: Vec2<f32>,
    },
    SpawnIn {
        position: Vec2<f32>,
    },
    Shadow {
        position: Vec2<f32>,
        scale: f32,
    },
    Platform {
        position: Vec2<f32>,
        width: f32,
    },
}

/// Whether a [Sprite] is visible or not.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Visibility {
    /// Sprite is shown.
    #[default]
    Shown,
    /// Sprite is not shown, but still exists in DOM.
    Hidden,
}

/// Component for entities with a 2D costume
#[derive(Debug)]
pub struct Sprite {
    /// Type of sprite. It cannot changed after initialization
    pub costume: Costume,
    /// Whether the sprite is visible.
    pub visibility: Visibility,
    /// Handle of the `platform`'s object(for drop management).
    pub handle: Option<u32>,
}

impl Sprite {
    /// Create a new sprite component
    pub fn new(costume: Costume) -> Self {
        Self {
            costume,
            visibility: Default::default(),
            handle: None,
        }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        Canvas::remove(self);
    }
}

/// Component for a shadow entity
pub struct Shadow(pub Entity);

/// System that animates the sprites of shadows
pub fn animate_shadow_sprites(world: &mut World) {
    let mut rm = Vec::new();
    for (e, (shadow, sprite)) in &mut world.query::<(&Shadow, &mut Sprite)>() {
        let Costume::Shadow { position, scale } = &mut sprite.costume else {
            continue;
        };
        let Ok(transform) = world.get::<&Transform>(shadow.0) else {
            rm.push(e);
            continue;
        };
        if let Some((_, pos)) = physics::raycast_solid(
            world,
            transform.translation,
            vec2!(0.0, -1.0),
            Some(shadow.0)
        ) {
            let target = 35.0 * ((pos - transform.translation).magnitude() + 1.0).powi(-1);
            *position += 0.9 * (pos - *position);
            *scale += 0.7 * (target - *scale);
        }
    }
    for e in rm {
        world.despawn(e).unwrap();
    }
}

/// System that animates player sprites' squash/stretch
pub fn animate_player_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, sprite)) in world.query_mut::<(&Transform, &mut Sprite)>() {
        let Costume::Player { position, scale, lean, .. } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        let delta = target - *position;

        let target_lean = delta.x;
        let target_scale = vec2!(
            1.0 - 0.01 * delta.y.abs(),
            1.0 + 0.02 * delta.y.abs()
        );
        let target_lean = target_lean.min(15.0).max(-15.0);
        let target_scale = target_scale.map(|n| n.max(0.5).min(2.0));
        
        *position += delta * 0.6;
        // Lean in direction of movement unless jumping/falling
        *lean += (target_lean - *lean) * 0.2;
        // Squash/stretch
        scale.x += (target_scale.x - scale.x) * 0.6;
        scale.y += (target_scale.y - scale.y) * 0.6;

        // Snap back
        if delta.norm_squared() > 10000.0 {
            *position = target;
            *lean = 0.0;
            *scale = vec2!(1.0, 1.0);
            log::info!("snap {}", delta.norm_squared());
        }
    }
}

/// System that animates bullets
pub fn animate_bullet_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, sprite)) in world.query_mut::<(&Transform, &mut Sprite)>() {
        let Costume::Bullet { position } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        // TODO: trail
        let _delta = target - *position;

        *position = target;
    }
}

pub fn animate_handheld_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, ability, sprite)) in world.query_mut::<(&Transform, &Ability, &mut Sprite)>() {
        let (position, rotation) = match &mut sprite.costume {
            Costume::Shotgun { position, rotation } => (position, rotation),
            Costume::AssaultRifle { position, rotation } => (position, rotation),
            Costume::DualGun { position, rotation } => (position, rotation),
            Costume::Shield { position, rotation } => (position, rotation),
            _ => {
                continue;
            }
        };
        let target = transform.translation;
        let delta = target - *position;

        // Damp position
        *position += 0.9 * delta;
        // Rotation is exact
        *rotation = transform.rotation;
        // Visibility
        sprite.visibility = match ability.active {
            true => Visibility::Shown,
            false => Visibility::Hidden,
        };
    }
}

pub fn animate_bubble_shield_sprite(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, ability, shield, cooldown, sprite)) in world.query_mut::<(
        &Transform, &Ability, &BubbleShield, &Cooldown, &mut Sprite
    )>() {
        let Costume::BubbleShield { position, radius } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        let delta = target - *position;

        // Damp position
        *position += 0.9 * delta;
        // Radius is exact
        *radius = shield.radius.max(0.0);
        // Visibility
        sprite.visibility = match ability.active && cooldown.0 <= 0.0 {
            true => Visibility::Shown,
            false => Visibility::Hidden,
        };
    }   
}

pub fn animate_health_bar_sprites(world: &mut World) {
    if cfg!(server) {
        return;
    }
    for (_, (transform, parent, sprite)) in &mut world.query::<(&Transform, &Parent, &mut Sprite)>() {
        let Costume::HealthBar { position, percentage } = &mut sprite.costume else {
            continue;
        };
        let target = transform.translation;
        let delta = target - *position;

        // Damp position
        *position += 0.9 * delta;
        // Percentage
        if let Ok(health) = world.get::<&Health>(parent.0) {
            *percentage = health.now / health.max;
        }
    }
}

/// System that draws sprites
pub fn draw_sprites(world: &mut World, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (_, sprite) in world.query_mut::<&mut Sprite>() {
        canvas.draw(sprite);
    }
}

/// System that updates cooldown UIs
pub fn draw_cooldowns(socket: &Socket, canvas: &Canvas) {
    if cfg!(server) {
        return;
    }
    for (_, packet) in socket.packets() {
        let Packet::CooldownStart { binding, duration } = packet else {
            continue;
        };
        canvas.set_cooldown(*binding, *duration);
    }
}
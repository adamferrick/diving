use crate::collision::*;
use crate::health::*;
use crate::position::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    damage: Damage,
    hitbox: RectangularHitbox,
    projectile: Projectile,
    velocity: Velocity,
}

impl ProjectileBundle {
    pub fn new(damage: f32, width: f32, height: f32, dx: f32, dy: f32) -> Self {
        Self {
            damage: Damage(damage),
            hitbox: RectangularHitbox(Rectangle::new(width, height)),
            projectile: Projectile,
            velocity: Velocity(Vec3::new(dx, dy, 0.)),
        }
    }
}

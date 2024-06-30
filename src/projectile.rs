use crate::collision::*;
use crate::health::*;
use crate::position::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Event)]
pub struct ProjectileHit {
    pub projectile: Entity,
    pub target: Entity,
}

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

pub fn projectile_plugin(app: &mut App) {
    app.add_event::<ProjectileHit>();
    app.add_systems(FixedUpdate, projectile_hit.after(projectile_collision));
}

pub fn projectile_hit(
    projectiles: Query<&Damage, With<Projectile>>,
    targets: Query<Entity, (With<Health>, Without<Dead>)>,
    mut hit_events: EventReader<ProjectileHit>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for hit_event in hit_events.read() {
        if let (Ok(damage), Ok(target)) = (
            projectiles.get(hit_event.projectile),
            targets.get(hit_event.target),
        ) {
            damage_events.send(DamageEvent {
                target: target,
                damage: damage.0,
            });
        }
    }
}

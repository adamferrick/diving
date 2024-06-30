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
    mut commands: Commands,
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
            commands.entity(hit_event.projectile).despawn();
        }
    }
}

#[test]
fn hit_target() {
    let mut app = App::new();
    app.add_event::<ProjectileHit>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, projectile_hit);
    const DAMAGE: f32 = 5.;
    let projectile_id = app.world.spawn((Projectile, Damage(DAMAGE))).id();
    let target_id = app.world.spawn((Health(10.),)).id();
    // Send hit event
    app.world
        .resource_mut::<Events<ProjectileHit>>()
        .send(ProjectileHit {
            projectile: projectile_id,
            target: target_id,
        });
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next().unwrap();
    // Should have sent a DamageEvent { target: target_id, damage: DAMAGE }
    assert_eq!(damage.target, target_id);
    assert_eq!(damage.damage, DAMAGE);
    // Projectile should have despawned
    assert_eq!(app.world.query::<&Projectile>().iter(&app.world).len(), 0);
}

#[test]
fn do_not_hit_dead_target() {
    let mut app = App::new();
    app.add_event::<ProjectileHit>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, projectile_hit);
    const DAMAGE: f32 = 5.;
    let projectile_id = app.world.spawn((Projectile, Damage(DAMAGE))).id();
    let target_id = app.world.spawn((Health(0.), Dead)).id();
    // Send hit event
    app.world
        .resource_mut::<Events<ProjectileHit>>()
        .send(ProjectileHit {
            projectile: projectile_id,
            target: target_id,
        });
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next();
    // Should be no damage events
    assert!(damage.is_none());
    // Projectile should not have despawned
    assert_eq!(app.world.query::<&Projectile>().iter(&app.world).len(), 1);
}

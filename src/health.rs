use crate::projectile::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Component)]
pub struct Dead;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: f32,
}

pub fn health_plugin(app: &mut App) {
    app.add_event::<DamageEvent>();
    app.add_systems(
        FixedUpdate,
        (
            damage_health.after(projectile_hit),
            kill.after(damage_health),
        ),
    );
}

pub fn damage_health(
    mut damagables: Query<&mut Health, Without<Dead>>,
    mut damage_events: EventReader<DamageEvent>,
) {
    for damage_event in damage_events.read() {
        if let Ok(mut health) = damagables.get_mut(damage_event.target) {
            health.0 -= damage_event.damage;
            println!(
                "damage dealt: {}, resulting health value: {}",
                damage_event.damage, health.0
            );
        }
    }
}

pub fn kill(mut commands: Commands, living: Query<(Entity, &Health), Without<Dead>>) {
    for (entity, health) in &living {
        if health.0 <= 0. {
            commands.entity(entity).insert(Dead);
            println!("killing entity");
        }
    }
}

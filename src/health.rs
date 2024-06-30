use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct Damage(pub f32);

#[derive(Event)]
pub struct DamageEvent(pub Entity, pub f32);

pub fn damage_health(mut damagables: Query<&mut Health>, mut damage_events: EventReader<DamageEvent>) {
    for damage_event in damage_events.read() {
        if let Ok(mut health) = damagables.get_mut(damage_event.0) {
            health.0 -= damage_event.1;
            println!("damage dealt: {}, resulting health value: {}", damage_event.1, health.0);
        }
    }
}

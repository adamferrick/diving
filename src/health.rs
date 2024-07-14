use crate::projectile::*;
use crate::states::RunningStateSet;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Damage(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
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
        )
            .in_set(RunningStateSet),
    );
    app.register_type::<Health>();
    app.register_type::<Damage>();
    app.register_type::<Dead>();
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

#[test]
fn did_damage() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, damage_health);
    let damagable_id = app.world_mut().spawn(Health(10.)).id();
    // send damage event
    app.world_mut()
        .resource_mut::<Events<DamageEvent>>()
        .send(DamageEvent {
            target: damagable_id,
            damage: 5.,
        });
    app.update();
    let new_health = app.world().get::<Health>(damagable_id).unwrap().0;
    assert_eq!(new_health, 5.);
}

#[test]
fn dont_damage_dead() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, damage_health);
    let damagable_id = app.world_mut().spawn((Health(0.), Dead)).id();
    // send damage event
    app.world_mut()
        .resource_mut::<Events<DamageEvent>>()
        .send(DamageEvent {
            target: damagable_id,
            damage: 5.,
        });
    app.update();
    let new_health = app.world().get::<Health>(damagable_id).unwrap().0;
    assert_eq!(new_health, 0.);
}

pub fn kill(mut commands: Commands, living: Query<(Entity, &Health), Without<Dead>>) {
    for (entity, health) in &living {
        if health.0 <= 0. {
            commands.entity(entity).insert(Dead);
            println!("killing entity");
        }
    }
}

#[test]
fn kill_entity() {
    let mut app = App::new();
    app.add_systems(Update, kill);
    let damagable_id = app.world_mut().spawn(Health(0.)).id();
    assert!(app.world().get::<Dead>(damagable_id).is_none());
    app.update();
    assert!(app.world().get::<Dead>(damagable_id).is_some());
}

#[test]
fn do_not_kill_entity() {
    let mut app = App::new();
    app.add_systems(Update, kill);
    let damagable_id = app.world_mut().spawn(Health(1.)).id();
    app.update();
    assert!(app.world().get::<Dead>(damagable_id).is_none());
}

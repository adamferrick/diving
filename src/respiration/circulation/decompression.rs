use crate::circulation::*;
use crate::health::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SafeOutgassingAmount(pub f32);

pub fn decompression_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (outgassing_damage.after(equalize_pressure),));
}
pub fn outgassing_damage(
    breathers: Query<&SafeOutgassingAmount, With<Health>>,
    mut bloodstream_outgassings: EventReader<Outgassing>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for bloodstream_outgassing in bloodstream_outgassings.read() {
        if let Ok(safe_outgassing_amount) = breathers.get(bloodstream_outgassing.entity) {
            if bloodstream_outgassing.amount > safe_outgassing_amount.0 {
                damage_events.send(DamageEvent {
                    target: bloodstream_outgassing.entity,
                    damage: bloodstream_outgassing.amount - safe_outgassing_amount.0,
                });
            }
        }
    }
}

#[test]
fn harmful_outgassing() {
    let mut app = App::new();
    app.add_event::<Outgassing>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, outgassing_damage);
    let breather_id = app
        .world
        .spawn((SafeOutgassingAmount(20.), Health(100.)))
        .id();
    app.world
        .resource_mut::<Events<Outgassing>>()
        .send(Outgassing {
            entity: breather_id,
            amount: 50.,
        });
    app.update();
    // should send a DamageEvent
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next().unwrap();
    assert_eq!(damage.target, breather_id);
    assert_eq!(damage.damage, 30.);
}

#[test]
fn harmless_outgassing() {
    let mut app = App::new();
    app.add_event::<Outgassing>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, outgassing_damage);
    let breather_id = app
        .world
        .spawn((SafeOutgassingAmount(20.), Health(100.)))
        .id();
    app.world
        .resource_mut::<Events<Outgassing>>()
        .send(Outgassing {
            entity: breather_id,
            amount: 15.,
        });
    app.update();
    // should not send a DamageEvent
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next();
    assert!(damage.is_none());
}

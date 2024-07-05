use crate::circulation::*;
use crate::health::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct GasExchangeInLungs {
    pub max_load: f32,
    pub load: f32,
    pub recovery_rate: f32,
}

pub fn decompression_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            outgassing_load.after(equalize_pressure),
            outgassing_deload.after(outgassing_load),
        ),
    );
}

pub fn outgassing_load(
    mut breathers: Query<&mut GasExchangeInLungs, With<Health>>,
    mut bloodstream_outgassings: EventReader<Outgassing>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for bloodstream_outgassing in bloodstream_outgassings.read() {
        if let Ok(mut lungs) = breathers.get_mut(bloodstream_outgassing.entity) {
            lungs.load += bloodstream_outgassing.amount;
            if lungs.load > lungs.max_load {
                damage_events.send(DamageEvent {
                    target: bloodstream_outgassing.entity,
                    damage: lungs.load - lungs.max_load,
                });
            }
            lungs.load = lungs.load.min(lungs.max_load);
        }
    }
}

#[test]
fn harmful_outgassing() {
    let mut app = App::new();
    app.add_event::<Outgassing>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, outgassing_load);
    let breather_id = app
        .world
        .spawn((
            GasExchangeInLungs {
                max_load: 2.,
                load: 0.,
                recovery_rate: 0.,
            },
            Health(100.),
        ))
        .id();
    app.world
        .resource_mut::<Events<Outgassing>>()
        .send(Outgassing {
            entity: breather_id,
            amount: 3.,
        });
    app.update();
    let lungs = app.world.get::<GasExchangeInLungs>(breather_id).unwrap();
    assert_eq!(lungs.load, 2.);
    // should send a DamageEvent
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next().unwrap();
    assert_eq!(damage.target, breather_id);
    assert_eq!(damage.damage, 1.);
}

#[test]
fn harmless_outgassing() {
    let mut app = App::new();
    app.add_event::<Outgassing>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, outgassing_load);
    let breather_id = app
        .world
        .spawn((
            GasExchangeInLungs {
                max_load: 2.,
                load: 0.,
                recovery_rate: 0.,
            },
            Health(100.),
        ))
        .id();
    app.world
        .resource_mut::<Events<Outgassing>>()
        .send(Outgassing {
            entity: breather_id,
            amount: 1.,
        });
    app.update();
    let lungs = app.world.get::<GasExchangeInLungs>(breather_id).unwrap();
    assert_eq!(lungs.load, 1.);
    // should not send a DamageEvent
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next();
    assert!(damage.is_none());
}

pub fn outgassing_deload(mut gas_exchangers: Query<&mut GasExchangeInLungs>) {
    for mut gas_exchanger in &mut gas_exchangers {
        gas_exchanger.load = (gas_exchanger.load - gas_exchanger.recovery_rate).max(0.);
    }
}

#[test]
fn did_deload() {
    let mut app = App::new();
    app.add_systems(Update, outgassing_deload);
    let breather_id = app
        .world
        .spawn(GasExchangeInLungs {
            max_load: 2.,
            load: 1.,
            recovery_rate: 0.5,
        })
        .id();
    app.update();
    let lungs = app.world.get::<GasExchangeInLungs>(breather_id).unwrap();
    assert_eq!(lungs.load, 0.5);
}

#[test]
fn fully_deloaded() {
    let mut app = App::new();
    app.add_systems(Update, outgassing_deload);
    let breather_id = app
        .world
        .spawn(GasExchangeInLungs {
            max_load: 2.,
            load: 0.,
            recovery_rate: 0.5,
        })
        .id();
    app.update();
    let lungs = app.world.get::<GasExchangeInLungs>(breather_id).unwrap();
    assert_eq!(lungs.load, 0.);
}

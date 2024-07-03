use crate::circulation::*;
use crate::health::*;
use crate::inhalation::*;
use crate::position::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InertGasInBloodstream(pub f32);

#[derive(Component)]
pub struct SafeOutgassingAmount(pub f32);

#[derive(Event)]
pub struct BloodstreamOutgassing {
    pub entity: Entity,
    pub amount: f32,
}

pub fn decompression_plugin(app: &mut App) {
    app.add_event::<BloodstreamOutgassing>();
    app.add_systems(
        FixedUpdate,
        (
            absorbing_and_outgassing,
            outgassing_damage.after(absorbing_and_outgassing),
        ),
    );
}

pub fn absorbing_and_outgassing(
    mut breathers: Query<(
        Entity,
        &Depth,
        &mut InertGasInBloodstream,
        &BloodstreamContent,
    )>,
    mut gases_to_circulate: EventReader<CirculateGas>,
    mut bloodstream_outgassing: EventWriter<BloodstreamOutgassing>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok((entity, depth, mut inert_gas_in_bloodstream, bloodstream_content)) =
            breathers.get_mut(gas_to_circulate.entity)
        {
            let delta = (gas_to_circulate.amount / bloodstream_content.capacity)
                * (depth.0 - inert_gas_in_bloodstream.0);
            inert_gas_in_bloodstream.0 += delta;
            println!("inert gas in bloodstream: {}", inert_gas_in_bloodstream.0);
            if delta < 0. {
                bloodstream_outgassing.send(BloodstreamOutgassing {
                    entity: entity,
                    amount: -delta,
                });
            }
        }
    }
}

#[test]
fn absorb_full_breath() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<CirculateGas>();
    app.add_event::<BloodstreamOutgassing>();
    let breather_id = app
        .world
        .spawn((
            Depth(100.),
            InertGasInBloodstream(0.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 100.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        });
    app.update();
    let inert_gas = app
        .world
        .get::<InertGasInBloodstream>(breather_id)
        .unwrap()
        .0;
    assert_eq!(inert_gas, 100.);
    // should not send a BloodstreamOutgassing event (gas was absorbed, not released)
    let bloodstream_outgassing_events = app.world.resource::<Events<BloodstreamOutgassing>>();
    let mut bloodstream_outgassing_reader = bloodstream_outgassing_events.get_reader();
    let bloodstream_outgassing = bloodstream_outgassing_reader
        .read(bloodstream_outgassing_events)
        .next();
    assert!(bloodstream_outgassing.is_none());
}

#[test]
fn absorb_partial_breath() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<CirculateGas>();
    app.add_event::<BloodstreamOutgassing>();
    let breather_id = app
        .world
        .spawn((
            Depth(100.),
            InertGasInBloodstream(0.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 50.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        });
    app.update();
    let inert_gas = app
        .world
        .get::<InertGasInBloodstream>(breather_id)
        .unwrap()
        .0;
    assert_eq!(inert_gas, 50.);
    // should not send a BloodstreamOutgassing event (gas was absorbed, not released)
    let bloodstream_outgassing_events = app.world.resource::<Events<BloodstreamOutgassing>>();
    let mut bloodstream_outgassing_reader = bloodstream_outgassing_events.get_reader();
    let bloodstream_outgassing = bloodstream_outgassing_reader
        .read(bloodstream_outgassing_events)
        .next();
    assert!(bloodstream_outgassing.is_none());
}

#[test]
fn outgas_full_breath() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<CirculateGas>();
    app.add_event::<BloodstreamOutgassing>();
    let breather_id = app
        .world
        .spawn((
            Depth(0.),
            InertGasInBloodstream(100.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 100.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        });
    app.update();
    let inert_gas = app
        .world
        .get::<InertGasInBloodstream>(breather_id)
        .unwrap()
        .0;
    assert_eq!(inert_gas, 0.);
    // should not send a BloodstreamOutgassing event (gas was absorbed, not released)
    let bloodstream_outgassing_events = app.world.resource::<Events<BloodstreamOutgassing>>();
    let mut bloodstream_outgassing_reader = bloodstream_outgassing_events.get_reader();
    let bloodstream_outgassing = bloodstream_outgassing_reader
        .read(bloodstream_outgassing_events)
        .next()
        .unwrap();
    assert_eq!(bloodstream_outgassing.entity, breather_id);
    assert_eq!(bloodstream_outgassing.amount, 100.);
}

#[test]
fn outgas_partial_breath() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<CirculateGas>();
    app.add_event::<BloodstreamOutgassing>();
    let breather_id = app
        .world
        .spawn((
            Depth(0.),
            InertGasInBloodstream(100.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 50.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        });
    app.update();
    let inert_gas = app
        .world
        .get::<InertGasInBloodstream>(breather_id)
        .unwrap()
        .0;
    assert_eq!(inert_gas, 50.);
    // should not send a BloodstreamOutgassing event (gas was absorbed, not released)
    let bloodstream_outgassing_events = app.world.resource::<Events<BloodstreamOutgassing>>();
    let mut bloodstream_outgassing_reader = bloodstream_outgassing_events.get_reader();
    let bloodstream_outgassing = bloodstream_outgassing_reader
        .read(bloodstream_outgassing_events)
        .next()
        .unwrap();
    assert_eq!(bloodstream_outgassing.entity, breather_id);
    assert_eq!(bloodstream_outgassing.amount, 50.);
}

pub fn outgassing_damage(
    breathers: Query<&SafeOutgassingAmount, With<Health>>,
    mut bloodstream_outgassings: EventReader<BloodstreamOutgassing>,
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
    app.add_event::<BloodstreamOutgassing>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, outgassing_damage);
    let breather_id = app
        .world
        .spawn((SafeOutgassingAmount(20.), Health(100.)))
        .id();
    app.world
        .resource_mut::<Events<BloodstreamOutgassing>>()
        .send(BloodstreamOutgassing {
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
    app.add_event::<BloodstreamOutgassing>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, outgassing_damage);
    let breather_id = app
        .world
        .spawn((SafeOutgassingAmount(20.), Health(100.)))
        .id();
    app.world
        .resource_mut::<Events<BloodstreamOutgassing>>()
        .send(BloodstreamOutgassing {
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

use crate::circulation::*;
use crate::inhalation::*;
use crate::position::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InertGasInBloodstream(pub f32);

#[derive(Component)]
pub struct SafeOutgassingAmount(pub f32);

#[derive(Event)]
pub struct BloodstreamOutgassing(pub f32);

pub fn decompression_plugin(app: &mut App) {
    app.add_event::<BloodstreamOutgassing>();
    app.add_systems(FixedUpdate, absorbing_and_outgassing);
}

pub fn absorbing_and_outgassing(
    mut breathers: Query<(&Depth, &mut InertGasInBloodstream, &Lungs)>,
    mut gases_to_circulate: EventReader<CirculateGas>,
    mut bloodstream_outgassing: EventWriter<BloodstreamOutgassing>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok((depth, mut inert_gas_in_bloodstream, lungs)) =
            breathers.get_mut(gas_to_circulate.entity)
        {
            let delta =
                (gas_to_circulate.amount / lungs.capacity) * (depth.0 - inert_gas_in_bloodstream.0);
            inert_gas_in_bloodstream.0 += delta;
            println!("inert gas in bloodstream: {}", inert_gas_in_bloodstream.0);
            if delta < 0. {
                bloodstream_outgassing.send(BloodstreamOutgassing(-delta));
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
            Lungs {
                capacity: 100.,
                amount_remaining: 100.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 100.,
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
            Lungs {
                capacity: 100.,
                amount_remaining: 100.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 50.,
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
            Lungs {
                capacity: 100.,
                amount_remaining: 100.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 100.,
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
    assert_eq!(bloodstream_outgassing.0, 100.);
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
            Lungs {
                capacity: 100.,
                amount_remaining: 100.,
            },
        ))
        .id();
    app.world
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 50.,
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
    assert_eq!(bloodstream_outgassing.0, 50.);
}

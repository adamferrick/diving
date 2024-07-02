use crate::circulation::*;
use crate::inhalation::*;
use crate::position::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InertGasInBloodstream(pub f32);

#[derive(Component)]
pub struct SafeOutgassingRate(pub f32);

pub fn decompression_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, absorbing_and_outgassing);
}

pub fn absorbing_and_outgassing(
    mut breathers: Query<(&Depth, &mut InertGasInBloodstream, &Lungs)>,
    mut gases_to_circulate: EventReader<CirculateGas>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok((depth, mut inert_gas_in_bloodstream, lungs)) =
            breathers.get_mut(gas_to_circulate.entity)
        {
            inert_gas_in_bloodstream.0 +=
                (gas_to_circulate.amount / lungs.capacity) * (depth.0 - inert_gas_in_bloodstream.0);
            println!("inert gas in bloodstream: {}", inert_gas_in_bloodstream.0);
        }
    }
}

#[test]
fn full_breath() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<CirculateGas>();
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
}

#[test]
fn partial_breath() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<CirculateGas>();
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
}

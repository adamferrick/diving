use bevy::prelude::*;

use crate::respiration::circulation::*;
use crate::respiration::BloodstreamContent;

pub fn intake_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, intake_gas);
}

pub fn intake_gas(
    mut breathers: Query<&mut BloodstreamContent>,
    mut gases_to_circulate: EventReader<CirculateGas>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok(mut bloodstream_content) = breathers.get_mut(gas_to_circulate.entity) {
            bloodstream_content.amount_remaining = (bloodstream_content.amount_remaining
                + gas_to_circulate.amount)
                .min(bloodstream_content.capacity);
        }
    }
}

#[test]
fn did_intake_gas() {
    let mut app = App::new();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, intake_gas);
    let breather_id = app
        .world
        .spawn(BloodstreamContent {
            capacity: 100.,
            amount_remaining: 50.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        })
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
    let new_bloodstream = app.world.get::<BloodstreamContent>(breather_id).unwrap();
    assert_eq!(new_bloodstream.amount_remaining, 100.);
}

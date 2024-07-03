use bevy::prelude::*;

use crate::respiration::circulation::*;
use crate::respiration::BloodstreamContent;
use crate::Depth;

#[derive(Component)]
pub struct BloodstreamPressure(pub f32);

pub fn equalization_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, equalize_pressure);
}

fn weighted_average(value_1: f32, weight_1: f32, value_2: f32, weight_2: f32) -> f32 {
    (weight_1 * value_1 + weight_2 * value_2) / (weight_1 + weight_2)
}

pub fn equalize_pressure(
    mut breathers: Query<(&mut BloodstreamPressure, &BloodstreamContent, &Depth)>,
    mut gases_to_circulate: EventReader<CirculateGas>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok((mut bloodstream_pressure, bloodstream_content, depth)) =
            breathers.get_mut(gas_to_circulate.entity)
        {
            bloodstream_pressure.0 = weighted_average(
                bloodstream_pressure.0,
                bloodstream_content.amount_remaining,
                depth.0,
                gas_to_circulate.amount,
            );
        }
    }
}

#[test]
fn did_equalize_pressure() {
    let mut app = App::new();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, equalize_pressure);
    let breather_id = app
        .world
        .spawn((
            BloodstreamPressure(0.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 75.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
            Depth(100.),
        ))
        .id();
    app.world.resource_mut::<Events<CirculateGas>>().send(CirculateGas {
        entity: breather_id,
        amount: 25.,
        proportion_of_oxygen: 0.,
        proportion_of_nitrogen: 0.,
    });
    app.update();
    let new_bloodstream_pressure = app.world.get::<BloodstreamPressure>(breather_id).unwrap();
    assert_eq!(new_bloodstream_pressure.0, 25.);
}

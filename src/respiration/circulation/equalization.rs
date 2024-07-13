use bevy::prelude::*;

use crate::position::Depth;
use crate::respiration::circulation::*;
use crate::respiration::BloodstreamContent;
use crate::states::PausedState;

const ATMOSPHERIC_PRESSURE_BAR: f32 = 1.;
const BAR_MSW_RATIO: f32 = 1. / 10.;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BloodstreamPressure(pub f32);

impl Default for BloodstreamPressure {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Event)]
pub struct Outgassing {
    pub entity: Entity,
    pub amount: f32,
}

pub fn equalization_plugin(app: &mut App) {
    app.add_event::<Outgassing>();
    app.add_systems(
        FixedUpdate,
        (equalize_pressure, equalize_gases)
            .run_if(in_state(PausedState::Running))
            .before(crate::respiration::circulation::intake::intake_gas),
    );
    app.register_type::<BloodstreamPressure>();
}

fn weighted_average(value_1: f32, weight_1: f32, value_2: f32, weight_2: f32) -> f32 {
    (weight_1 * value_1 + weight_2 * value_2) / (weight_1 + weight_2)
}

pub fn equalize_pressure(
    mut breathers: Query<(
        Entity,
        &mut BloodstreamPressure,
        &BloodstreamContent,
        &Depth,
    )>,
    mut gases_to_circulate: EventReader<CirculateGas>,
    mut outgassings: EventWriter<Outgassing>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok((breather_entity, mut bloodstream_pressure, bloodstream_content, depth)) =
            breathers.get_mut(gas_to_circulate.entity)
        {
            let pressure_at_current_depth = ATMOSPHERIC_PRESSURE_BAR + BAR_MSW_RATIO * depth.0;
            let new_bloodstream_pressure = weighted_average(
                bloodstream_pressure.0,
                bloodstream_content.amount_remaining,
                pressure_at_current_depth,
                gas_to_circulate.amount,
            );
            let delta = new_bloodstream_pressure - bloodstream_pressure.0;
            if delta < 0. {
                outgassings.send(Outgassing {
                    entity: breather_entity,
                    amount: -delta,
                });
            }
            bloodstream_pressure.0 = new_bloodstream_pressure;
        }
    }
}

#[test]
fn did_equalize_pressure_absorption() {
    let mut app = App::new();
    app.add_event::<CirculateGas>();
    app.add_event::<Outgassing>();
    app.add_systems(Update, equalize_pressure);
    let breather_id = app
        .world_mut()
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
    app.world_mut()
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 25.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        });
    app.update();
    // should have equalized the pressure
    let new_bloodstream_pressure = app.world().get::<BloodstreamPressure>(breather_id).unwrap();
    assert_eq!(new_bloodstream_pressure.0, 2.75);
    // should not have sent an outgassing event (gas should have been absorbed)
    let outgassing_events = app.world().resource::<Events<Outgassing>>();
    let mut outgassing_reader = outgassing_events.get_reader();
    let outgassing = outgassing_reader.read(outgassing_events).next();
    assert!(outgassing.is_none());
}

#[test]
fn did_equalize_pressure_outgassing() {
    let mut app = App::new();
    app.add_event::<CirculateGas>();
    app.add_event::<Outgassing>();
    app.add_systems(Update, equalize_pressure);
    let breather_id = app
        .world_mut()
        .spawn((
            BloodstreamPressure(2.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 75.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
            Depth(0.),
        ))
        .id();
    app.world_mut()
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 25.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        });
    app.update();
    let new_bloodstream_pressure = app.world().get::<BloodstreamPressure>(breather_id).unwrap();
    assert_eq!(new_bloodstream_pressure.0, 1.75);
    // should have sent an outgassing event
    let outgassing_events = app.world().resource::<Events<Outgassing>>();
    let mut outgassing_reader = outgassing_events.get_reader();
    let outgassing = outgassing_reader.read(outgassing_events).next().unwrap();
    assert_eq!(outgassing.entity, breather_id);
    assert_eq!(outgassing.amount, 0.25);
}

pub fn equalize_gases(
    mut breathers: Query<&mut BloodstreamContent>,
    mut gases_to_circulate: EventReader<CirculateGas>,
) {
    for gas_to_circulate in gases_to_circulate.read() {
        if let Ok(mut bloodstream_content) = breathers.get_mut(gas_to_circulate.entity) {
            bloodstream_content.proportion_of_oxygen = weighted_average(
                bloodstream_content.proportion_of_oxygen,
                bloodstream_content.amount_remaining,
                gas_to_circulate.proportion_of_oxygen,
                gas_to_circulate.amount,
            );
            bloodstream_content.proportion_of_nitrogen = weighted_average(
                bloodstream_content.proportion_of_nitrogen,
                bloodstream_content.amount_remaining,
                gas_to_circulate.proportion_of_nitrogen,
                gas_to_circulate.amount,
            );
        }
    }
}

#[test]
fn did_equalize_gases() {
    let mut app = App::new();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, equalize_gases);
    let breather_id = app
        .world_mut()
        .spawn(BloodstreamContent {
            capacity: 100.,
            amount_remaining: 50.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        })
        .id();
    app.world_mut()
        .resource_mut::<Events<CirculateGas>>()
        .send(CirculateGas {
            entity: breather_id,
            amount: 50.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.5,
        });
    app.update();
    let new_bloodstream_content = app.world().get::<BloodstreamContent>(breather_id).unwrap();
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 0.25);
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 0.25);
}

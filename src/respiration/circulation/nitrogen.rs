use bevy::prelude::*;

use crate::respiration::circulation::BloodstreamPressure;
use crate::respiration::BloodstreamContent;
use crate::DamageEvent;

const PN2_NARCOSIS_THRESHOLD: f32 = 0.78 * (30. / 10. + 1.);

#[derive(Component)]
pub struct NitrogenHazard {
    n2_upper: f32,
    damage_factor: f32,
}

impl Default for NitrogenHazard {
    fn default() -> Self {
        Self {
            n2_upper: PN2_NARCOSIS_THRESHOLD,
            damage_factor: 0.,
        }
    }
}

pub fn nitrogen_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, nitrogen_narcosis);
}

pub fn nitrogen_narcosis(
    breathers: Query<(
        Entity,
        &NitrogenHazard,
        &BloodstreamPressure,
        &BloodstreamContent,
    )>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (entity, nitrogen_hazard, bloodstream_pressure, bloodstream_content) in &breathers {
        let pressure_from_nitrogen =
            bloodstream_content.proportion_of_nitrogen * bloodstream_pressure.0;
        if pressure_from_nitrogen > nitrogen_hazard.n2_upper {
            damage_events.send(DamageEvent {
                target: entity,
                damage: nitrogen_hazard.damage_factor
                    * (pressure_from_nitrogen - nitrogen_hazard.n2_upper),
            });
        }
    }
}

#[test]
fn did_narcosis_damage() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, nitrogen_narcosis);
    let breather_id = app
        .world
        .spawn((
            NitrogenHazard {
                n2_upper: 2.,
                damage_factor: 1.,
            },
            BloodstreamPressure(3.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 1.,
            },
        ))
        .id();
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next().unwrap();
    assert_eq!(damage.target, breather_id);
    assert_eq!(damage.damage, 1.);
}

#[test]
fn did_no_narcosis_damage() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, nitrogen_narcosis);
    app.world.spawn((
        NitrogenHazard {
            n2_upper: 2.,
            damage_factor: 1.,
        },
        BloodstreamPressure(1.),
        BloodstreamContent {
            capacity: 100.,
            amount_remaining: 100.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 1.,
        },
    ));
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next();
    assert!(damage.is_none());
}

use bevy::prelude::*;

use crate::respiration::circulation::BloodstreamPressure;
use crate::respiration::BloodstreamContent;
use crate::DamageEvent;

#[derive(Component)]
pub struct OxygenHazard {
    po2_upper: f32,
    po2_lower: f32,
    damage_factor: f32,
}

pub fn oxygen_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, oxygen_damage);
}

pub fn oxygen_damage(
    breathers: Query<(
        Entity,
        &OxygenHazard,
        &BloodstreamPressure,
        &BloodstreamContent,
    )>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (entity, toxicity, bloodstream_pressure, bloodstream_content) in &breathers {
        let pressure_from_oxygen =
            bloodstream_content.proportion_of_oxygen * bloodstream_pressure.0;
        if pressure_from_oxygen > toxicity.po2_upper {
            damage_events.send(DamageEvent {
                target: entity,
                damage: toxicity.damage_factor * (pressure_from_oxygen - toxicity.po2_upper),
            });
        } else if pressure_from_oxygen < toxicity.po2_lower {
            damage_events.send(DamageEvent {
                target: entity,
                damage: toxicity.damage_factor * (toxicity.po2_lower - pressure_from_oxygen),
            });
        }
    }
}

#[test]
fn did_toxicity_damage() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, oxygen_damage);
    let breather_id = app
        .world
        .spawn((
            OxygenHazard {
                po2_upper: 3.,
                po2_lower: 1.,
                damage_factor: 1.,
            },
            BloodstreamPressure(4.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 1.,
                proportion_of_nitrogen: 0.,
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
fn did_no_oxygen_damage() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, oxygen_damage);
    app.world.spawn((
        OxygenHazard {
            po2_upper: 3.,
            po2_lower: 1.,
            damage_factor: 1.,
        },
        BloodstreamPressure(2.),
        BloodstreamContent {
            capacity: 100.,
            amount_remaining: 100.,
            proportion_of_oxygen: 1.,
            proportion_of_nitrogen: 0.,
        },
    ));
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next();
    assert!(damage.is_none());
}

#[test]
fn did_hypoxia_damage() {
    let mut app = App::new();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, oxygen_damage);
    let breather_id = app
        .world
        .spawn((
            OxygenHazard {
                po2_upper: 3.,
                po2_lower: 2.,
                damage_factor: 1.,
            },
            BloodstreamPressure(1.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 1.,
                proportion_of_nitrogen: 0.,
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

use crate::respiration::circulation::intake::*;
use crate::respiration::BloodstreamContent;
use bevy::prelude::*;

#[derive(Component)]
pub struct GasUsageRate(pub f32);

pub fn usage_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            usage.after(intake_gas),
            update_proportions_on_exhaust.after(usage),
        ),
    );
}

pub fn usage(mut gas_users: Query<(&GasUsageRate, &mut BloodstreamContent)>) {
    for (gas_usage_rate, mut bloodstream_content) in &mut gas_users {
        bloodstream_content.amount_remaining =
            (bloodstream_content.amount_remaining - gas_usage_rate.0).max(0.);
    }
}

#[test]
fn did_use() {
    let mut app = App::new();
    app.add_systems(Update, usage);
    let gas_user_id = app
        .world
        .spawn((
            GasUsageRate(50.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 100.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    app.update();
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(gas_user_id).unwrap();
    assert_eq!(new_bloodstream_content.amount_remaining, 50.);
}

#[test]
fn did_use_exhaust() {
    let mut app = App::new();
    app.add_systems(Update, usage);
    let gas_user_id = app
        .world
        .spawn((
            GasUsageRate(50.),
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 25.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    app.update();
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(gas_user_id).unwrap();
    assert_eq!(new_bloodstream_content.amount_remaining, 0.);
}

pub fn update_proportions_on_exhaust(mut bloodstream_contents: Query<&mut BloodstreamContent>) {
    for mut bloodstream_content in &mut bloodstream_contents {
        if bloodstream_content.amount_remaining <= 0. {
            bloodstream_content.proportion_of_oxygen = 0.;
        }
    }
}

#[test]
fn did_update_proportions_on_exhaust() {
    let mut app = App::new();
    app.add_systems(Update, update_proportions_on_exhaust);
    let gas_user_id = app
        .world
        .spawn((BloodstreamContent {
            capacity: 100.,
            amount_remaining: 0.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.,
        },))
        .id();
    app.update();
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(gas_user_id).unwrap();
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 0.);
}
#[test]
fn did_not_update_proportions_on_non_exhaust() {
    let mut app = App::new();
    app.add_systems(Update, update_proportions_on_exhaust);
    let gas_user_id = app
        .world
        .spawn((BloodstreamContent {
            capacity: 100.,
            amount_remaining: 25.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.,
        },))
        .id();
    app.update();
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(gas_user_id).unwrap();
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 0.5);
}

use crate::circulation::CirculateGas;
use crate::helpers::weighted_average;
use bevy::prelude::*;

#[derive(Component)]
pub struct DivingCylinder {
    pub capacity: f32,
    pub amount_remaining: f32,
    pub proportion_of_oxygen: f32,
    pub proportion_of_nitrogen: f32,
}

#[derive(Component)]
pub struct EquippedTank(pub Entity);

#[derive(Component)]
pub struct BloodstreamContent {
    pub capacity: f32,
    pub amount_remaining: f32,
    pub proportion_of_oxygen: f32,
    pub proportion_of_nitrogen: f32,
}

#[derive(Event)]
pub struct BreathTaken {
    pub entity: Entity,
}

pub fn inhalation_plugin(app: &mut App) {
    app.add_event::<BreathTaken>();
    app.add_systems(FixedUpdate, inhalation);
}

pub fn inhalation(
    mut breathers: Query<(Entity, &mut BloodstreamContent, &EquippedTank)>,
    mut cylinders: Query<&mut DivingCylinder>,
    mut breaths: EventReader<BreathTaken>,
    mut circulate_gas: EventWriter<CirculateGas>,
) {
    for breath in breaths.read() {
        if let Ok((entity, mut bloodstream_content, equipped_tank_id)) =
            breathers.get_mut(breath.entity)
        {
            if let Ok(mut cylinder) = cylinders.get_mut(equipped_tank_id.0) {
                let amount_breathed = (bloodstream_content.capacity
                    - bloodstream_content.amount_remaining)
                    .min(cylinder.amount_remaining);
                cylinder.amount_remaining -= amount_breathed;
                bloodstream_content.proportion_of_oxygen = weighted_average(
                    bloodstream_content.proportion_of_oxygen,
                    bloodstream_content.amount_remaining,
                    cylinder.proportion_of_oxygen,
                    amount_breathed,
                );
                bloodstream_content.proportion_of_nitrogen = weighted_average(
                    bloodstream_content.proportion_of_nitrogen,
                    bloodstream_content.amount_remaining,
                    cylinder.proportion_of_nitrogen,
                    amount_breathed,
                );
                bloodstream_content.amount_remaining += amount_breathed;
                println!(
                    "amount breathed: {}, tank remaining: {}, bloodstream remaining: {}, oxygen: {}, nitrogen: {}",
                    amount_breathed,
                    cylinder.amount_remaining,
                    bloodstream_content.amount_remaining,
                    bloodstream_content.proportion_of_oxygen,
                    bloodstream_content.proportion_of_nitrogen,
                );
                if amount_breathed > 0. {
                    circulate_gas.send(CirculateGas {
                        entity: entity,
                        amount: amount_breathed,
                    });
                }
            }
        }
    }
}

#[test]
fn saturate_bloodstream() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 100.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.5,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 50.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
            EquippedTank(cylinder_id),
        ))
        .id();
    app.world
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    // bloodstream_content proportion should be full
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(breather_id).unwrap();
    assert_eq!(new_bloodstream_content.amount_remaining, 100.);
    // bloodstream should be a fourth oxygen and a fourth nitrogen
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 0.25);
    assert_eq!(new_bloodstream_content.proportion_of_nitrogen, 0.25);
    // cylinder proportion should be half empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 50.);
    // should have sent an event
    let gas_to_circulate_events = app.world.resource::<Events<CirculateGas>>();
    let mut gas_to_circulate_reader = gas_to_circulate_events.get_reader();
    let gas_to_circulate = gas_to_circulate_reader
        .read(gas_to_circulate_events)
        .next()
        .unwrap();
    assert_eq!(gas_to_circulate.entity, breather_id);
    assert_eq!(gas_to_circulate.amount, 50.);
}

#[test]
fn saturate_bloodstream_partial() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 50.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.5,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 25.,
                proportion_of_oxygen: 0.0,
                proportion_of_nitrogen: 0.0,
            },
            EquippedTank(cylinder_id),
        ))
        .id();
    app.world
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    // bloodstream_content proportion should be 3/4ths full
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(breather_id).unwrap();
    assert_eq!(new_bloodstream_content.amount_remaining, 75.);
    // bloodstream content should be a third oxygen and a third nitrogen
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 1. / 3.);
    assert_eq!(new_bloodstream_content.proportion_of_nitrogen, 1. / 3.);
    // cylinder proportion should be empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 0.);
    // should have sent an event
    let gas_to_circulate_events = app.world.resource::<Events<CirculateGas>>();
    let mut gas_to_circulate_reader = gas_to_circulate_events.get_reader();
    let gas_to_circulate = gas_to_circulate_reader
        .read(gas_to_circulate_events)
        .next()
        .unwrap();
    assert_eq!(gas_to_circulate.entity, breather_id);
    assert_eq!(gas_to_circulate.amount, 50.);
}

#[test]
fn empty_cylinder() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 0.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 50.,
                proportion_of_oxygen: 0.5,
                proportion_of_nitrogen: 0.5,
            },
            EquippedTank(cylinder_id),
        ))
        .id();
    app.world
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    // bloodstream_content proportion should unchanged
    let new_bloodstream_content = app.world.get::<BloodstreamContent>(breather_id).unwrap();
    assert_eq!(new_bloodstream_content.amount_remaining, 50.);
    // should still be half oxygen and half nitrogen
    assert_eq!(new_bloodstream_content.proportion_of_oxygen, 0.5);
    assert_eq!(new_bloodstream_content.proportion_of_nitrogen, 0.5);
    // cylinder proportion still should be empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 0.);
    // should not have sent an event
    let gas_to_circulate_events = app.world.resource::<Events<CirculateGas>>();
    let mut gas_to_circulate_reader = gas_to_circulate_events.get_reader();
    let gas_to_circulate = gas_to_circulate_reader.read(gas_to_circulate_events).next();
    assert!(gas_to_circulate.is_none());
}

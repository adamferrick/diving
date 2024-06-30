use bevy::prelude::*;

#[derive(Component)]
pub struct DivingCylinder {
    pub capacity: f32,
    pub amount_remaining: f32,
    pub proportion_of_oxygen: f32,
}

#[derive(Component)]
pub struct EquippedTank(pub Entity);

#[derive(Component)]
pub struct Lungs {
    pub capacity: f32,
    pub amount_remaining: f32,
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
    mut breathers: Query<(&mut Lungs, &EquippedTank)>,
    mut cylinders: Query<&mut DivingCylinder>,
    mut breaths: EventReader<BreathTaken>,
) {
    for breath in breaths.read() {
        if let Ok((mut lungs, equipped_tank_id)) = breathers.get_mut(breath.entity) {
            if let Ok(mut cylinder) = cylinders.get_mut(equipped_tank_id.0) {
                let amount_breathed =
                    (lungs.capacity - lungs.amount_remaining).min(cylinder.amount_remaining);
                cylinder.amount_remaining -= amount_breathed;
                lungs.amount_remaining += amount_breathed;
                println!(
                    "amount breathed: {}, tank remaining: {}, lung remaining: {}",
                    amount_breathed, cylinder.amount_remaining, lungs.amount_remaining
                );
            }
        }
    }
}

#[test]
fn fill_lungs() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 100.,
            proportion_of_oxygen: 0.21,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            Lungs {
                capacity: 100.,
                amount_remaining: 50.,
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
    // lungs proportion should be full
    let new_lungs = app.world.get::<Lungs>(breather_id).unwrap();
    assert_eq!(new_lungs.amount_remaining, 100.);
    // cylinder proportion should be half empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 50.);
}

#[test]
fn fill_lungs_partial() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 50.,
            proportion_of_oxygen: 0.21,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            Lungs {
                capacity: 100.,
                amount_remaining: 25.,
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
    // lungs proportion should be 3/4ths full
    let new_lungs = app.world.get::<Lungs>(breather_id).unwrap();
    assert_eq!(new_lungs.amount_remaining, 75.);
    // cylinder proportion should be empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 0.);
}

#[test]
fn empty_cylinder() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 0.,
            proportion_of_oxygen: 0.21,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            Lungs {
                capacity: 100.,
                amount_remaining: 50.,
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
    // lungs proportion should unchanged
    let new_lungs = app.world.get::<Lungs>(breather_id).unwrap();
    assert_eq!(new_lungs.amount_remaining, 50.);
    // cylinder proportion still should be empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 0.);
}

use bevy::prelude::*;

#[derive(Component)]
pub struct DivingCylinder {
    pub capacity: f32,
    pub proportion_remaining: f32,
    pub proportion_of_oxygen: f32,
}

#[derive(Component)]
pub struct EquippedTank(pub Entity);

#[derive(Component)]
pub struct Lungs {
    pub capacity: f32,
    pub proportion_remaining: f32,
}

#[derive(Event)]
pub struct BreathTaken {
    pub entity: Entity,
}

pub fn inhalation(
    mut breathers: Query<(&mut Lungs, &EquippedTank)>,
    mut cylinders: Query<&mut DivingCylinder>,
    mut breaths: EventReader<BreathTaken>,
) {
    for breath in breaths.read() {
        if let Ok((mut lungs, equipped_tank_id)) = breathers.get_mut(breath.entity) {
            if let Ok(mut cylinder) = cylinders.get_mut(equipped_tank_id.0) {
                let amount_breathed = (lungs.capacity * (1. - lungs.proportion_remaining))
                    .min(cylinder.capacity * cylinder.proportion_remaining);
                cylinder.proportion_remaining -= amount_breathed / cylinder.capacity;
                lungs.proportion_remaining += amount_breathed / lungs.capacity;
                println!("amount breathed: {}, tank proportion remaining: {}, lung proportion remaining: {}", amount_breathed, cylinder.proportion_remaining, lungs.proportion_remaining);
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
            proportion_remaining: 1.,
            proportion_of_oxygen: 0.21,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            Lungs {
                capacity: 100.,
                proportion_remaining: 0.5,
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
    assert_eq!(new_lungs.proportion_remaining, 1.);
    // cylinder proportion should be half empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.proportion_remaining, 0.5);
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
            proportion_remaining: 0.5,
            proportion_of_oxygen: 0.21,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            Lungs {
                capacity: 100.,
                proportion_remaining: 0.25,
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
    assert_eq!(new_lungs.proportion_remaining, 0.75);
    // cylinder proportion should be empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.proportion_remaining, 0.);
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
            proportion_remaining: 0.,
            proportion_of_oxygen: 0.21,
        })
        .id();
    let breather_id = app
        .world
        .spawn((
            Lungs {
                capacity: 100.,
                proportion_remaining: 0.5,
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
    assert_eq!(new_lungs.proportion_remaining, 0.5);
    // cylinder proportion still should be empty
    let new_cylinder = app.world.get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.proportion_remaining, 0.);
}

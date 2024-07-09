use bevy::prelude::*;

use crate::inhalation::*;

#[derive(Component)]
pub struct Equippable;

#[derive(Component)]
pub struct Equipped(pub Entity);

#[derive(Event)]
pub struct CylinderEquipEvent {
    pub item: Entity,
    pub wearer: Entity,
}

pub fn equipment_plugin(app: &mut App) {
    app.add_event::<CylinderEquipEvent>();
    app.add_systems(FixedUpdate, equip_cylinder);
}

pub fn equip_cylinder(
    mut commands: Commands,
    mut wearers: Query<&mut EquippedTank>,
    mut cylinder_equip_events: EventReader<CylinderEquipEvent>,
) {
    for cylinder_equip_event in cylinder_equip_events.read() {
        if let Ok(mut equipped_tank) = wearers.get_mut(cylinder_equip_event.wearer) {
            commands.entity(equipped_tank.0).remove::<Equipped>();
            equipped_tank.0 = cylinder_equip_event.item;
        } else {
            commands
                .entity(cylinder_equip_event.wearer)
                .insert(EquippedTank(cylinder_equip_event.item));
        }
        commands
            .entity(cylinder_equip_event.item)
            .insert(Equipped(cylinder_equip_event.wearer));
    }
}

#[test]
fn did_equip_cylinder() {
    let mut app = App::new();
    app.add_event::<CylinderEquipEvent>();
    app.add_systems(Update, equip_cylinder);
    let cylinder_id = app
        .world
        .spawn((
            Equippable,
            DivingCylinder {
                capacity: 0.,
                amount_remaining: 0.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    let wearer_id = app.world.spawn(()).id();
    app.world
        .resource_mut::<Events<CylinderEquipEvent>>()
        .send(CylinderEquipEvent {
            item: cylinder_id,
            wearer: wearer_id,
        });
    app.update();
    let equipped = app.world.get::<Equipped>(cylinder_id).unwrap();
    assert_eq!(equipped.0, wearer_id);
    let equipped_tank = app.world.get::<EquippedTank>(wearer_id).unwrap();
    assert_eq!(equipped_tank.0, cylinder_id);
}

#[test]
fn did_replace_cylinder() {
    let mut app = App::new();
    app.add_event::<CylinderEquipEvent>();
    app.add_systems(Update, equip_cylinder);
    let cylinder_1_id = app
        .world
        .spawn((
            Equippable,
            DivingCylinder {
                capacity: 0.,
                amount_remaining: 0.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    let cylinder_2_id = app
        .world
        .spawn((
            Equippable,
            DivingCylinder {
                capacity: 0.,
                amount_remaining: 0.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
        ))
        .id();
    let wearer_id = app.world.spawn(EquippedTank(cylinder_1_id)).id();
    app.world.get_entity_mut(cylinder_1_id).unwrap().insert(Equipped(wearer_id));
    app.world
        .resource_mut::<Events<CylinderEquipEvent>>()
        .send(CylinderEquipEvent {
            item: cylinder_2_id,
            wearer: wearer_id,
        });
    app.update();
    // cylinder 2 should be equipped
    let equipped = app.world.get::<Equipped>(cylinder_2_id).unwrap();
    assert_eq!(equipped.0, wearer_id);
    let equipped_tank = app.world.get::<EquippedTank>(wearer_id).unwrap();
    assert_eq!(equipped_tank.0, cylinder_2_id);
    // cylinder 1 should not be equipped
    assert!(app.world.get::<Equipped>(cylinder_1_id).is_none());
}

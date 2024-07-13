use bevy::prelude::*;

use crate::inhalation::*;
use crate::states::PausedState;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Equippable;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Equipped(pub Entity);

#[derive(Event)]
pub struct CylinderEquipEvent {
    pub item: Entity,
    pub wearer: Entity,
}

#[derive(Event)]
pub struct CylinderUnequipEvent {
    pub wearer: Entity,
}

pub fn equipment_plugin(app: &mut App) {
    app.add_event::<CylinderEquipEvent>();
    app.add_event::<CylinderUnequipEvent>();
    app.add_systems(
        FixedUpdate,
        (equip_cylinder, unequip_cylinder).run_if(in_state(PausedState::Running)),
    );
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
        .world_mut()
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
    let wearer_id = app.world_mut().spawn(()).id();
    app.world_mut()
        .resource_mut::<Events<CylinderEquipEvent>>()
        .send(CylinderEquipEvent {
            item: cylinder_id,
            wearer: wearer_id,
        });
    app.update();
    let equipped = app.world().get::<Equipped>(cylinder_id).unwrap();
    assert_eq!(equipped.0, wearer_id);
    let equipped_tank = app.world().get::<EquippedTank>(wearer_id).unwrap();
    assert_eq!(equipped_tank.0, cylinder_id);
}

#[test]
fn did_replace_cylinder() {
    let mut app = App::new();
    app.add_event::<CylinderEquipEvent>();
    app.add_systems(Update, equip_cylinder);
    let cylinder_1_id = app
        .world_mut()
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
        .world_mut()
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
    let wearer_id = app.world_mut().spawn(EquippedTank(cylinder_1_id)).id();
    app.world_mut()
        .get_entity_mut(cylinder_1_id)
        .unwrap()
        .insert(Equipped(wearer_id));
    app.world_mut()
        .resource_mut::<Events<CylinderEquipEvent>>()
        .send(CylinderEquipEvent {
            item: cylinder_2_id,
            wearer: wearer_id,
        });
    app.update();
    // cylinder 2 should be equipped
    let equipped = app.world().get::<Equipped>(cylinder_2_id).unwrap();
    assert_eq!(equipped.0, wearer_id);
    let equipped_tank = app.world().get::<EquippedTank>(wearer_id).unwrap();
    assert_eq!(equipped_tank.0, cylinder_2_id);
    // cylinder 1 should not be equipped
    assert!(app.world().get::<Equipped>(cylinder_1_id).is_none());
}

pub fn unequip_cylinder(
    mut commands: Commands,
    equipped_cylinders: Query<&EquippedTank>,
    mut cylinder_unequip_events: EventReader<CylinderUnequipEvent>,
) {
    for cylinder_unequip_event in cylinder_unequip_events.read() {
        if let Ok(equipped_cylinder) = equipped_cylinders.get(cylinder_unequip_event.wearer) {
            if let Some(mut cylinder_entity) = commands.get_entity(equipped_cylinder.0) {
                cylinder_entity.remove::<Equipped>();
            }
        }
        if let Some(mut wearer_entity) = commands.get_entity(cylinder_unequip_event.wearer) {
            wearer_entity.remove::<EquippedTank>();
        }
    }
}

#[test]
fn did_unequip_cylinder() {
    let mut app = App::new();
    app.add_event::<CylinderUnequipEvent>();
    app.add_systems(Update, unequip_cylinder);
    let cylinder_id = app
        .world_mut()
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
    let wearer_id = app.world_mut().spawn(EquippedTank(cylinder_id)).id();
    app.world_mut()
        .get_entity_mut(cylinder_id)
        .unwrap()
        .insert(Equipped(wearer_id));
    app.world_mut()
        .resource_mut::<Events<CylinderUnequipEvent>>()
        .send(CylinderUnequipEvent { wearer: wearer_id });
    app.update();
    let wearer = app.world().get::<EquippedTank>(wearer_id);
    assert!(wearer.is_none());
    let worn_cylinder = app.world().get::<Equipped>(cylinder_id);
    assert!(worn_cylinder.is_none());
}

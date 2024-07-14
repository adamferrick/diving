use bevy::prelude::*;

use crate::diver::Diver;
use crate::inhalation::*;
use crate::states::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Equippable;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Equipped(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EquipmentMenu;

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
        (equip_cylinder, unequip_cylinder).in_set(RunningStateSet),
    );
    app.add_systems(Update, toggle_inventory);
    app.add_systems(OnEnter(InGameMenuState::Inventory), spawn_equipment_menu);
    app.add_systems(OnExit(InGameMenuState::Inventory), despawn_equipment_menu);
    app.register_type::<Equippable>();
    app.register_type::<Equipped>();
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

pub fn toggle_inventory(
    game_state: Res<State<GameState>>,
    in_game_menu_state: Res<State<InGameMenuState>>,
    mut next_in_game_menu_state: ResMut<NextState<InGameMenuState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyI) && *game_state.get() == GameState::Running {
        match in_game_menu_state.get() {
            InGameMenuState::Inventory => next_in_game_menu_state.set(InGameMenuState::NoMenu),
            _ => next_in_game_menu_state.set(InGameMenuState::Inventory),
        }
    }
}

pub fn spawn_equipment_menu(
    mut commands: Commands,
    equipped_cylinder: Query<&EquippedTank, With<Diver>>,
    names: Query<&Name>,
) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(50.),
            height: Val::Percent(50.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Srgba::rgb(0., 0., 1.).into(),
        ..default()
    };
    let cylinder_name = match equipped_cylinder.get_single() {
        Ok(equipped) => match names.get(equipped.0) {
            Ok(name) => name,
            _ => "UNNAMED ENTITY",
        },
        _ => "",
    };
    let message = TextBundle {
        text: Text::from_section(
            format!("Cylinder: {}", cylinder_name),
            TextStyle {
                font_size: crate::FONT_SIZE,
                ..default()
            },
        ),
        ..default()
    };
    let container_id = commands
        .spawn((container, EquipmentMenu, Name::new("Equipment menu")))
        .id();
    let message_id = commands.spawn(message).id();
    commands.entity(container_id).push_children(&[message_id]);
}

pub fn despawn_equipment_menu(
    mut commands: Commands,
    equipment_menus: Query<Entity, With<EquipmentMenu>>,
) {
    if let Ok(equipment_menu) = equipment_menus.get_single() {
        commands.entity(equipment_menu).despawn_recursive();
    }
}

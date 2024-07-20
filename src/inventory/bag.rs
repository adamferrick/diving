use bevy::prelude::*;

use crate::inventory::inventory_menu::*;
use crate::inventory_menu::InventoryMenu;
use crate::states::*;
use crate::Diver;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Collectible;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Collected(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Bag {
    pub collectibles: Vec<Entity>,
    pub capacity: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BagMenu;

#[derive(Event)]
pub struct ItemPickup {
    pub item: Entity,
    pub bag: Entity,
}

#[derive(Event)]
pub struct ItemDrop {
    pub item: Entity,
}

pub fn bag_plugin(app: &mut App) {
    app.add_event::<ItemPickup>();
    app.add_event::<ItemDrop>();
    app.add_systems(
        FixedUpdate,
        (pick_up_item, drop_item).in_set(RunningStateSet),
    );
    app.add_systems(
        OnEnter(InGameMenuState::Inventory),
        spawn_bag_menu.after(spawn_inventory_menu),
    );
    app.register_type::<Collectible>();
    app.register_type::<Collected>();
    app.register_type::<Bag>();
}

pub fn pick_up_item(
    mut commands: Commands,
    items: Query<Entity, (With<Collectible>, Without<Collected>)>,
    mut bags: Query<&mut Bag>,
    mut item_pickups: EventReader<ItemPickup>,
) {
    for pickup in item_pickups.read() {
        if let (Ok(item), Ok(mut bag)) = (items.get(pickup.item), bags.get_mut(pickup.bag)) {
            if !bag.collectibles.iter().any(|id| *id == pickup.item)
                && bag.collectibles.len() < bag.capacity
            {
                bag.collectibles.push(pickup.item);
                commands.entity(item).insert(Collected(pickup.bag));
            }
        }
    }
}

#[test]
fn did_pick_up() {
    let mut app = App::new();
    app.add_event::<ItemPickup>();
    app.add_systems(Update, pick_up_item);
    let bag_id = app
        .world_mut()
        .spawn(Bag {
            collectibles: Vec::new(),
            capacity: 2,
        })
        .id();
    let item_id = app.world_mut().spawn(Collectible).id();
    app.world_mut()
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_id,
            bag: bag_id,
        });
    app.update();
    let collected = app.world().get::<Collected>(item_id).unwrap();
    assert_eq!(collected.0, bag_id);
    let bag = app.world().get::<Bag>(bag_id).unwrap();
    assert_eq!(bag.collectibles[0], item_id);
    assert_eq!(bag.collectibles.len(), 1);
    // should not be able to pick up items twice
    app.world_mut()
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_id,
            bag: bag_id,
        });
    let bag = app.world().get::<Bag>(bag_id).unwrap();
    assert_eq!(bag.collectibles.len(), 1);
}

#[test]
fn did_not_pick_up_no_capacity() {
    let mut app = App::new();
    app.add_event::<ItemPickup>();
    app.add_systems(Update, pick_up_item);
    let bag_id = app
        .world_mut()
        .spawn(Bag {
            collectibles: Vec::new(),
            capacity: 1,
        })
        .id();
    let item_1_id = app.world_mut().spawn(Collectible).id();
    let item_2_id = app.world_mut().spawn(Collectible).id();
    app.world_mut()
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_1_id,
            bag: bag_id,
        });
    app.update();
    app.world_mut()
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_2_id,
            bag: bag_id,
        });
    app.update();
    let bag = app.world().get::<Bag>(bag_id).unwrap();
    assert_eq!(bag.collectibles[0], item_1_id);
    assert_eq!(bag.collectibles.len(), 1);
}

pub fn drop_item(
    mut commands: Commands,
    items: Query<&Collected, With<Collectible>>,
    mut bags: Query<&mut Bag>,
    mut item_drops: EventReader<ItemDrop>,
) {
    for drop in item_drops.read() {
        if let Ok(item) = items.get(drop.item) {
            if let Ok(mut bag) = bags.get_mut(item.0) {
                bag.collectibles.retain(|item_id| *item_id != drop.item);
            }
            commands.entity(drop.item).remove::<Collected>();
        }
    }
}

#[test]
fn did_drop() {
    let mut app = App::new();
    app.add_event::<ItemDrop>();
    app.add_systems(Update, drop_item);
    let bag_id = app
        .world_mut()
        .spawn(Bag {
            collectibles: Vec::new(),
            capacity: 2,
        })
        .id();
    let item_id = app.world_mut().spawn((Collectible, Collected(bag_id))).id();
    app.world_mut()
        .get_mut::<Bag>(bag_id)
        .unwrap()
        .collectibles
        .push(item_id);
    app.world_mut()
        .resource_mut::<Events<ItemDrop>>()
        .send(ItemDrop { item: item_id });
    app.update();
}

pub fn spawn_bag_menu(
    mut commands: Commands,
    diver_bags: Query<&Bag, With<Diver>>,
    names: Query<&Name>,
    mut bag_menus: Query<Entity, With<BagMenu>>,
    inventory_menus: Query<Entity, With<InventoryMenu>>,
) {
    for bag_menu in &mut bag_menus {
        if let Some(mut bag_menu_commands) = commands.get_entity(bag_menu) {
            bag_menu_commands.despawn();
        }
    }
    if let Ok(bag) = diver_bags.get_single() {
        let item_node_ids: Vec<_> = bag
            .collectibles
            .iter()
            .map(|item_id| {
                let item_name = match names.get(*item_id) {
                    Ok(name) => name,
                    _ => "UNNAMED ENTITY",
                };
                let item = TextBundle {
                    text: Text::from_section(
                        item_name,
                        TextStyle {
                            font_size: crate::FONT_SIZE,
                            ..default()
                        },
                    ),
                    ..default()
                };
                commands.spawn(item).id()
            })
            .collect();
        let container = NodeBundle {
            style: Style {
                width: Val::Percent(50.),
                height: Val::Percent(90.),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(20.)),
                ..default()
            },
            background_color: Srgba::rgb(0., 0., 0.).into(),
            ..default()
        };
        let container_id = commands
            .spawn((container, BagMenu, Name::new("Bag Menu")))
            .push_children(&item_node_ids)
            .id();
        if let Ok(inventory_menu) = inventory_menus.get_single() {
            commands
                .entity(inventory_menu)
                .push_children(&[container_id]);
        }
    }
}

use crate::inventory::collectible::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bag {
    pub collectibles: Vec<Entity>,
    pub capacity: usize,
}

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
    app.add_systems(FixedUpdate, pick_up_item);
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
        .world
        .spawn(Bag {
            collectibles: Vec::new(),
            capacity: 2,
        })
        .id();
    let item_id = app.world.spawn(Collectible).id();
    app.world
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_id,
            bag: bag_id,
        });
    app.update();
    let collected = app.world.get::<Collected>(item_id).unwrap();
    assert_eq!(collected.0, bag_id);
    let bag = app.world.get::<Bag>(bag_id).unwrap();
    assert_eq!(bag.collectibles[0], item_id);
    assert_eq!(bag.collectibles.len(), 1);
    // should not be able to pick up items twice
    app.world
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_id,
            bag: bag_id,
        });
    let bag = app.world.get::<Bag>(bag_id).unwrap();
    assert_eq!(bag.collectibles.len(), 1);
}

#[test]
fn did_not_pick_up_no_capacity() {
    let mut app = App::new();
    app.add_event::<ItemPickup>();
    app.add_systems(Update, pick_up_item);
    let bag_id = app
        .world
        .spawn(Bag {
            collectibles: Vec::new(),
            capacity: 1,
        })
        .id();
    let item_1_id = app.world.spawn(Collectible).id();
    let item_2_id = app.world.spawn(Collectible).id();
    app.world
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_1_id,
            bag: bag_id,
        });
    app.update();
    app.world
        .resource_mut::<Events<ItemPickup>>()
        .send(ItemPickup {
            item: item_2_id,
            bag: bag_id,
        });
    app.update();
    let bag = app.world.get::<Bag>(bag_id).unwrap();
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
        .world
        .spawn(Bag {
            collectibles: Vec::new(),
            capacity: 2,
        })
        .id();
    let item_id = app.world.spawn((Collectible, Collected(bag_id))).id();
    app.world
        .get_mut::<Bag>(bag_id)
        .unwrap()
        .collectibles
        .push(item_id);
    app.world
        .resource_mut::<Events<ItemDrop>>()
        .send(ItemDrop { item: item_id });
    app.update();
}

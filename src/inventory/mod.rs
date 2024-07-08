use bevy::prelude::*;

use crate::inventory::bag::bag_plugin;

pub mod bag;

pub fn inventory_plugin(app: &mut App) {
    app.add_plugins(bag_plugin);
}

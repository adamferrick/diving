use bevy::prelude::*;

use crate::inventory::bag::bag_plugin;

pub mod bag;
pub mod collectible;

pub fn inventory_plugin(app: &mut App) {
    app.add_plugins(bag_plugin);
}

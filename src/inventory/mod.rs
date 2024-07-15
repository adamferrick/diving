use bevy::prelude::*;

use crate::inventory::bag::bag_plugin;
use crate::inventory::equipment::equipment_plugin;
use crate::inventory::inventory_menu::inventory_menu_plugin;

pub mod bag;
pub mod equipment;
pub mod inventory_menu;

pub fn inventory_plugin(app: &mut App) {
    app.add_plugins((bag_plugin, equipment_plugin, inventory_menu_plugin));
}

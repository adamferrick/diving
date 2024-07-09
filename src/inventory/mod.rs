use bevy::prelude::*;

use crate::inventory::bag::bag_plugin;
use crate::inventory::equipment::equipment_plugin;

pub mod bag;
pub mod equipment;

pub fn inventory_plugin(app: &mut App) {
    app.add_plugins((bag_plugin, equipment_plugin));
}

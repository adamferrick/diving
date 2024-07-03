use bevy::prelude::*;

pub mod decompression;
pub mod pressure;

use decompression::*;
use pressure::*;

#[derive(Event)]
pub struct CirculateGas {
    pub entity: Entity,
    pub amount: f32,
}

pub fn circulation_plugin(app: &mut App) {
    app.add_event::<CirculateGas>();
    app.add_plugins((decompression_plugin, pressure_plugin));
}

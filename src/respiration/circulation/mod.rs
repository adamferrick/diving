use bevy::prelude::*;

pub mod decompression;
pub mod equalization;
pub mod intake;
pub mod usage;

use decompression::*;
use equalization::*;
use intake::*;
use usage::*;

#[derive(Event)]
pub struct CirculateGas {
    pub entity: Entity,
    pub amount: f32,
    pub proportion_of_oxygen: f32,
    pub proportion_of_nitrogen: f32,
}

pub fn circulation_plugin(app: &mut App) {
    app.add_event::<CirculateGas>();
    app.add_plugins((
        decompression_plugin,
        equalization_plugin,
        intake_plugin,
        usage_plugin,
    ));
}

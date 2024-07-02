use bevy::prelude::*;

pub mod decompression;

use decompression::*;

pub fn circulation_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, absorbing_and_outgassing);
}

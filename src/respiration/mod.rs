use bevy::prelude::*;

pub mod inhalation;

use inhalation::*;

pub fn respiration_plugin(app: &mut App) {
    app.add_plugins(inhalation_plugin);
}

use bevy::prelude::*;

pub mod circulation;
pub mod inhalation;

use crate::circulation::*;
use inhalation::*;

pub fn respiration_plugin(app: &mut App) {
    app.add_plugins((inhalation_plugin, circulation_plugin));
}

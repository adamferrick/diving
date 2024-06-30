use bevy::prelude::*;

pub mod inhalation;

use inhalation::*;

pub fn respiration_plugin(app: &mut App) {
    app.add_event::<BreathTaken>();
    app.add_systems(FixedUpdate, inhalation.after(crate::diver::player_inhale));
}

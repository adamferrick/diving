use bevy::prelude::*;

use collision::*;
use drag::*;
use position::*;

pub mod collision;
pub mod drag;
pub mod position;

pub fn physics_plugin(app: &mut App) {
    app.add_plugins((drag_plugin, collision_plugin, position_plugin));
}

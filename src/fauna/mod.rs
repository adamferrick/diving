use bevy::prelude::*;
use enemy::*;

pub mod enemy;

pub fn fauna_plugin(app: &mut App) {
    app.add_plugins(enemy_plugin);
}

use crate::camera::*;
use crate::diver::*;
use crate::fauna::*;
use crate::health::*;
use crate::inventory::*;
use crate::physics::*;
use crate::projectile::*;
use crate::respiration::*;
use crate::states::*;
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod camera;
pub mod diver;
pub mod fauna;
pub mod health;
pub mod inventory;
pub mod physics;
pub mod projectile;
pub mod respiration;
pub mod states;
pub mod ui;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CursorPosition(Vec2);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            health_plugin,
            diver_plugin,
            physics_plugin,
            projectile_plugin,
            inventory_plugin,
            respiration_plugin,
            states_plugin,
            fauna_plugin,
            ui_plugin,
            camera_plugin,
        ))
        .init_resource::<CursorPosition>()
        .add_systems(FixedUpdate, (update_cursor,))
        .run();
}

fn update_cursor(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    if let Some(new_cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_position.0 = new_cursor_position;
    }
}

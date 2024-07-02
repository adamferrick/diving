use crate::collision::*;
use crate::diver::*;
use crate::health::*;
use crate::position::*;
use crate::projectile::*;
use crate::respiration::*;
use crate::ui::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod collision;
pub mod diver;
pub mod health;
pub mod position;
pub mod projectile;
pub mod respiration;
pub mod ui;

#[derive(Resource, Default)]
pub struct CursorPosition(Vec2);

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            health_plugin,
            collision_plugin,
            diver_plugin,
            position_plugin,
            projectile_plugin,
            respiration_plugin,
            ui_plugin,
        ))
        .init_resource::<CursorPosition>()
        .add_systems(Startup, spawn_camera)
        .add_systems(FixedUpdate, (update_cursor,))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
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

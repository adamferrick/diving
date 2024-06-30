use crate::collision::*;
use crate::diver::*;
use crate::health::*;
use crate::position::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod collision;
pub mod diver;
pub mod health;
pub mod position;
pub mod projectile;

#[derive(Resource, Default)]
pub struct CursorPosition(Vec2);

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CursorPosition>()
        .add_event::<DamageEvent>()
        .add_systems(Startup, (spawn_camera, spawn_diver, spawn_obstacles))
        .add_systems(
            FixedUpdate,
            (
                update_cursor,
                player_control_velocity,
                update_position
                    .before(fire_speargun)
                    .after(player_control_velocity),
                obstacle_collision.after(update_position),
                projectile_collision.after(update_position),
                damage_health.after(projectile_collision),
                kill.after(damage_health),
                fire_speargun.after(update_cursor),
            ),
        )
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

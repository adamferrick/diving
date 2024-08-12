use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

use crate::states::*;
use bevy::prelude::*;

pub const SEA_LEVEL: f32 = 0.;
const METERS_TRANSLATION_RATIO: f32 = 10.;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Depth(pub f32);

pub fn position_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_position, update_depth.after(update_position))
            .run_if(in_state(PausedState::Running)),
    );
    app.register_type::<Velocity>();
    app.register_type::<Depth>();
}

pub fn update_position(mut movables: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut movables {
        transform.translation += velocity.0;
    }
}

#[test]
fn did_update_position() {
    let mut app = App::new();
    app.add_systems(Update, update_position);

    let movable_id = app
        .world_mut()
        .spawn((
            Transform::from_translation(Vec3::ZERO),
            Velocity(Vec3::new(1., 1., 0.)),
        ))
        .id();

    app.update();
    let new_translation = app
        .world()
        .get::<Transform>(movable_id)
        .unwrap()
        .translation;

    assert!(new_translation == Vec3::new(1., 1., 0.));
}

pub fn update_depth(mut submerged_objects: Query<(&mut Depth, &Transform)>) {
    for (mut depth, transform) in &mut submerged_objects {
        depth.0 = (SEA_LEVEL - transform.translation.y).max(0.) / METERS_TRANSLATION_RATIO;
    }
}

#[test]
fn depth_below_sea_level() {
    let mut app = App::new();
    app.add_systems(Update, update_depth);

    let movable_id = app
        .world_mut()
        .spawn((
            Transform::from_translation(Vec3::new(0., SEA_LEVEL - 100., 0.)),
            Depth(0.),
        ))
        .id();

    app.update();
    let new_depth = app.world().get::<Depth>(movable_id).unwrap().0;

    assert!(new_depth == 100. / METERS_TRANSLATION_RATIO);
}

#[test]
fn depth_above_sea_level() {
    let mut app = App::new();
    app.add_systems(Update, update_depth);

    let movable_id = app
        .world_mut()
        .spawn((
            Transform::from_translation(Vec3::new(0., SEA_LEVEL + 100., 0.)),
            Depth(0.),
        ))
        .id();

    app.update();
    let new_depth = app.world().get::<Depth>(movable_id).unwrap().0;

    assert!(new_depth == 0.);
}

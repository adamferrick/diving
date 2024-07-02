use bevy::prelude::*;

pub const SEA_LEVEL: f32 = 0.;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Depth(pub f32);

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
        .world
        .spawn((
            Transform::from_translation(Vec3::ZERO),
            Velocity(Vec3::new(1., 1., 0.)),
        ))
        .id();

    app.update();
    let new_translation = app.world.get::<Transform>(movable_id).unwrap().translation;

    assert!(new_translation == Vec3::new(1., 1., 0.));
}

pub fn update_depth(mut submerged_objects: Query<(&mut Depth, &Transform)>) {
    for (mut depth, transform) in &mut submerged_objects {
        depth.0 = (SEA_LEVEL - transform.translation.y).max(0.);
    }
}

#[test]
fn depth_below_sea_level() {
    let mut app = App::new();
    app.add_systems(Update, update_depth);

    let movable_id = app
        .world
        .spawn((
            Transform::from_translation(Vec3::new(0., SEA_LEVEL - 100., 0.)),
            Depth(0.),
        ))
        .id();

    app.update();
    let new_depth = app.world.get::<Depth>(movable_id).unwrap().0;

    assert!(new_depth == 100.);
}

#[test]
fn depth_above_sea_level() {
    let mut app = App::new();
    app.add_systems(Update, update_depth);

    let movable_id = app
        .world
        .spawn((
            Transform::from_translation(Vec3::new(0., SEA_LEVEL + 100., 0.)),
            Depth(0.),
        ))
        .id();

    app.update();
    let new_depth = app.world.get::<Depth>(movable_id).unwrap().0;

    assert!(new_depth == 0.);
}

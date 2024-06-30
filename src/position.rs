use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

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

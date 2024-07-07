use crate::Diver;
use crate::Health;
use crate::RectangularHitbox;
use crate::Velocity;
use bevy::prelude::*;

const ENEMY_SPEED: f32 = 1.1;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    hitbox: RectangularHitbox,
    health: Health,
    velocity: Velocity,
}

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(FixedUpdate, enemy_seek_diver);
}

pub fn enemy_seek_diver(
    divers: Query<&Transform, With<Diver>>,
    mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
) {
    if let Ok(diver_transform) = divers.get_single() {
        for (enemy_transform, mut enemy_velocity) in &mut enemies {
            enemy_velocity.0 = (diver_transform.translation - enemy_transform.translation)
                .normalize_or_zero()
                * ENEMY_SPEED;
        }
    }
}

#[test]
fn did_seek_diver() {
    let mut app = App::new();
    app.add_systems(Update, enemy_seek_diver);
    let enemy_id = app
        .world
        .spawn((
            Transform::from_translation(Vec3::ZERO),
            Velocity(Vec3::ZERO),
            Enemy,
        ))
        .id();
    app.world.spawn((
        Transform::from_translation(Vec3::new(1., 1., 0.)),
        Diver,
    ));
    app.update();
    let enemy_velocity = app.world.get::<Velocity>(enemy_id).unwrap();
    assert_eq!(enemy_velocity.0, Vec3::new(1., 1., 0.).normalize() * 1.1);
}

#[test]
fn enemy_on_diver() {
    let mut app = App::new();
    app.add_systems(Update, enemy_seek_diver);
    let enemy_id = app
        .world
        .spawn((
            Transform::from_translation(Vec3::ZERO),
            Velocity(Vec3::ZERO),
            Enemy,
        ))
        .id();
    app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        Diver,
    ));
    app.update();
    let enemy_velocity = app.world.get::<Velocity>(enemy_id).unwrap();
    assert_eq!(enemy_velocity.0, Vec3::ZERO);
}

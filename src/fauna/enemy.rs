use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::Dead;
use crate::Diver;
use crate::Health;
use crate::RectangularHitbox;
use crate::Velocity;
use crate::Drag;

const ENEMY_SPEED: f32 = 1.1;
const ENEMY_WIDTH: f32 = 20.;
const ENEMY_HEIGHT: f32 = 20.;
const ENEMY_HEALTH: f32 = 40.;
const ENEMY_DRAG: f32 = 0.99;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    hitbox: RectangularHitbox,
    health: Health,
    velocity: Velocity,
    drag: Drag,
}

impl EnemyBundle {
    fn new() -> Self {
        Self {
            enemy: Enemy,
            hitbox: RectangularHitbox(Rectangle::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
            health: Health(ENEMY_HEALTH),
            velocity: Velocity(Vec3::new(0., 0., 0.)),
            drag: Drag(ENEMY_DRAG),
        }
    }
}

pub fn enemy_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_enemies);
    app.add_systems(FixedUpdate, enemy_seek_diver);
}

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut spawn_enemy = |x: f32, y: f32| {
        let mesh = Mesh::from(Rectangle::new(ENEMY_WIDTH, ENEMY_HEIGHT));
        let material = ColorMaterial::from(Color::rgb(0., 0., 1.));
        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(material);
        commands.spawn((
            EnemyBundle::new(),
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                ..default()
            },
        ));
    };

    spawn_enemy(-100., 0.);
    spawn_enemy(0., -100.);
}

pub fn enemy_seek_diver(
    divers: Query<&Transform, With<Diver>>,
    mut enemies: Query<(&Transform, &mut Velocity), (With<Enemy>, Without<Dead>)>,
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
    app.world
        .spawn((Transform::from_translation(Vec3::new(1., 1., 0.)), Diver));
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
    app.world
        .spawn((Transform::from_translation(Vec3::ZERO), Diver));
    app.update();
    let enemy_velocity = app.world.get::<Velocity>(enemy_id).unwrap();
    assert_eq!(enemy_velocity.0, Vec3::ZERO);
}

#[test]
fn dead_enemy_not_seek() {
    let mut app = App::new();
    app.add_systems(Update, enemy_seek_diver);
    let enemy_id = app
        .world
        .spawn((
            Transform::from_translation(Vec3::ZERO),
            Velocity(Vec3::ZERO),
            Enemy,
            Dead,
        ))
        .id();
    app.world
        .spawn((Transform::from_translation(Vec3::new(1., 1., 0.)), Diver));
    app.update();
    let enemy_velocity = app.world.get::<Velocity>(enemy_id).unwrap();
    assert_eq!(enemy_velocity.0, Vec3::ZERO);
}

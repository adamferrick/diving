use crate::health::*;
use crate::position::*;
use crate::projectile::*;
use crate::states::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const OBSTACLE_WIDTH: f32 = 400.;
const OBSTACLE_HEIGHT: f32 = 100.;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RectangularHitbox(pub Rectangle);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Obstacle;

pub fn collision_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_obstacles);
    app.add_systems(
        FixedUpdate,
        (
            projectile_collision.after(update_position),
            obstacle_collision.after(update_position),
        )
            .run_if(in_state(GameState::Running)),
    );
    app.register_type::<RectangularHitbox>();
    app.register_type::<Obstacle>();
}

pub fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning obstacles...");

    let mesh = Mesh::from(Rectangle::new(OBSTACLE_WIDTH, OBSTACLE_HEIGHT));
    let material = ColorMaterial::from_color(Srgba::rgb(0., 0., 1.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        Obstacle,
        RectangularHitbox(Rectangle::new(OBSTACLE_WIDTH, OBSTACLE_HEIGHT)),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
            ..default()
        },
        Name::new("Obstacle"),
    ));
}

fn get_intersection_length(a_start: f32, a_end: f32, b_start: f32, b_end: f32) -> f32 {
    if b_start > a_end || a_start > b_end {
        0.
    } else {
        a_end.min(b_end) - a_start.max(b_start)
    }
}

fn get_collision_data(
    translation1: &Vec3,
    dims1: &RectangularHitbox,
    translation2: &Vec3,
    dims2: &RectangularHitbox,
) -> Option<(Vec3, f32)> {
    let x_penetration = get_intersection_length(
        translation1.x - dims1.0.half_size.x,
        translation1.x + dims1.0.half_size.x,
        translation2.x - dims2.0.half_size.x,
        translation2.x + dims2.0.half_size.x,
    );
    let y_penetration = get_intersection_length(
        translation1.y - dims1.0.half_size.y,
        translation1.y + dims1.0.half_size.y,
        translation2.y - dims2.0.half_size.y,
        translation2.y + dims2.0.half_size.y,
    );
    if x_penetration == 0. || y_penetration == 0. {
        return None;
    }
    let depth = x_penetration.min(y_penetration);
    if y_penetration < x_penetration {
        Some((
            Vec3::new(0., translation2.y - translation1.y, 0.).normalize(),
            depth,
        ))
    } else {
        Some((
            Vec3::new(translation2.x - translation1.x, 0., 0.).normalize(),
            depth,
        ))
    }
}

#[test]
fn do_collide_ident() {
    let translation1 = Vec3::ZERO;
    let dims1 = RectangularHitbox(Rectangle::new(1., 1.));
    let translation2 = Vec3::ZERO;
    let dims2 = RectangularHitbox(Rectangle::new(1., 1.));
    assert_ne!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        None
    );
}

#[test]
fn do_collide_up() {
    let translation1 = Vec3::ZERO;
    let dims1 = RectangularHitbox(Rectangle::new(1., 1.));
    let translation2 = Vec3::new(0., 0.75, 0.);
    let dims2 = RectangularHitbox(Rectangle::new(1., 1.));
    assert_eq!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        Some((Vec3::new(0., 1., 0.).normalize(), 0.25))
    );
}

#[test]
fn do_collide_left() {
    let translation1 = Vec3::ZERO;
    let dims1 = RectangularHitbox(Rectangle::new(1., 1.));
    let translation2 = Vec3::new(0.75, 0., 0.);
    let dims2 = RectangularHitbox(Rectangle::new(1., 1.));
    assert_eq!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        Some((Vec3::new(1., 0., 0.).normalize(), 0.25))
    );
}

#[test]
fn do_collide_diagonal() {
    let translation1 = Vec3::ZERO;
    let dims1 = RectangularHitbox(Rectangle::new(1., 1.));
    let translation2 = Vec3::new(0.75, 0.75, 0.);
    let dims2 = RectangularHitbox(Rectangle::new(1., 1.));
    assert_eq!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        Some((Vec3::new(1., 0., 0.).normalize(), 0.25))
    );
}

#[test]
fn dont_collide() {
    let translation1 = Vec3::new(20., -104., 0.);
    let dims1 = RectangularHitbox(Rectangle::new(10., 50.));
    let translation2 = Vec3::new(100., 100., 0.);
    let dims2 = RectangularHitbox(Rectangle::new(400., 100.));
    assert_eq!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        None
    );
}

#[test]
fn dont_collide_diagonal() {
    let translation1 = Vec3::ZERO;
    let dims1 = RectangularHitbox(Rectangle::new(0.5, 0.5));
    let translation2 = Vec3::ONE;
    let dims2 = RectangularHitbox(Rectangle::new(0.5, 0.5));
    assert_eq!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        None
    );
}

#[test]
fn dont_collide_left() {
    let translation1 = Vec3::ZERO;
    let dims1 = RectangularHitbox(Rectangle::new(0.5, 0.5));
    let translation2 = Vec3::new(1., 0., 0.);
    let dims2 = RectangularHitbox(Rectangle::new(0.5, 0.5));
    assert_eq!(
        get_collision_data(&translation1, &dims1, &translation2, &dims2),
        None
    );
}

pub fn obstacle_collision(
    mut moving_objects: Query<
        (&mut Transform, &RectangularHitbox, &mut Velocity),
        Without<Obstacle>,
    >,
    obstacles: Query<(&Transform, &RectangularHitbox), With<Obstacle>>,
) {
    for (mut moving_transform, moving_hitbox, mut velocity) in &mut moving_objects {
        for (obstacle_transform, obstacle_hitbox) in &obstacles {
            if let Some((normal, depth)) = get_collision_data(
                &obstacle_transform.translation,
                &obstacle_hitbox,
                &moving_transform.translation,
                &moving_hitbox,
            ) {
                println!("the moving object and obstacle are colliding. moving object transform: {}, obstacle transform: {}", moving_transform.translation, obstacle_transform.translation);
                if normal.x != 0. {
                    velocity.0.x *= -1.;
                } else if normal.y != 0. {
                    velocity.0.y *= -1.;
                }
                moving_transform.translation += depth * normal;
            }
        }
    }
}

pub fn projectile_collision(
    projectiles: Query<(Entity, &Transform, &RectangularHitbox), With<Projectile>>,
    targets: Query<(Entity, &Transform, &RectangularHitbox), With<Health>>,
    mut hit_event: EventWriter<ProjectileHit>,
) {
    for (projectile_entity, projectile_transform, projectile_hitbox) in &projectiles {
        for (target_entity, target_transform, target_hitbox) in &targets {
            if let Some(_) = get_collision_data(
                &projectile_transform.translation,
                &projectile_hitbox,
                &target_transform.translation,
                &target_hitbox,
            ) {
                hit_event.send(ProjectileHit {
                    projectile: projectile_entity,
                    target: target_entity,
                });
            }
        }
    }
}

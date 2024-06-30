use crate::collision::*;
use crate::health::*;
use crate::position::*;
use crate::projectile::*;
use crate::respiration::inhalation::*;
use crate::CursorPosition;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const DIVER_SPEED: f32 = 1.;
const DIVER_WIDTH: f32 = 20.;
const DIVER_HEIGHT: f32 = 20.;

const SPEAR_SIZE: f32 = 5.;
const SPEAR_INITIAL_VELOCITY: f32 = 5.;
const SPEAR_DAMAGE: f32 = 40.;
const SPEAR_FIRE_RADIUS: f32 = 40.;

const DIVER_TANK_CAPACITY: f32 = 1000.;
const DIVER_TANK_PROPORTION_REMAINING: f32 = 0.8;
const DIVER_TANK_PROPORTION_OXYGEN: f32 = 0.21;
const DIVER_LUNG_CAPACITY: f32 = 100.;
const DIVER_LUNG_PROPORTION_REMAINING: f32 = 0.5;

#[derive(Component)]
pub struct Diver;

#[derive(Bundle)]
pub struct DiverBundle {
    diver: Diver,
    hitbox: RectangularHitbox,
    health: Health,
    velocity: Velocity,
    equipped_tank: EquippedTank,
    lungs: Lungs,
}

impl DiverBundle {
    fn new() -> Self {
        Self {
            diver: Diver,
            hitbox: RectangularHitbox(Rectangle::new(DIVER_WIDTH, DIVER_HEIGHT)),
            health: Health(100.),
            velocity: Velocity(Vec3::new(0., 0., 0.)),
            equipped_tank: EquippedTank {
                capacity: DIVER_TANK_CAPACITY,
                proportion_remaining: DIVER_TANK_PROPORTION_REMAINING,
                proportion_of_oxygen: DIVER_TANK_PROPORTION_OXYGEN,
            },
            lungs: Lungs {
                capacity: DIVER_LUNG_CAPACITY,
                proportion_remaining: DIVER_LUNG_PROPORTION_REMAINING,
            },
        }
    }
}

pub fn spawn_diver(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning diver...");

    let mesh = Mesh::from(Rectangle::new(DIVER_WIDTH, DIVER_HEIGHT));
    let material = ColorMaterial::from(Color::rgb(0., 1., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((
        DiverBundle::new(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(20., -25., 0.)),
            ..default()
        },
    ));
}

pub fn player_control_velocity(
    buttons: Res<ButtonInput<KeyCode>>,
    mut diver: Query<&mut Velocity, With<Diver>>,
) {
    if let Ok(mut velocity) = diver.get_single_mut() {
        if buttons.pressed(KeyCode::ArrowUp) {
            velocity.0.y = DIVER_SPEED;
        } else if buttons.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -DIVER_SPEED;
        } else {
            velocity.0.y = 0.;
        }

        if buttons.pressed(KeyCode::ArrowLeft) {
            velocity.0.x = -DIVER_SPEED;
        } else if buttons.pressed(KeyCode::ArrowRight) {
            velocity.0.x = DIVER_SPEED;
        } else {
            velocity.0.x = 0.;
        }
    }
}

pub fn fire_speargun(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cursor_position: Res<CursorPosition>,
    diver: Query<(&Transform, &Velocity), With<Diver>>,
) {
    if let Ok((transform, velocity)) = diver.get_single() {
        if buttons.just_pressed(MouseButton::Left) {
            let diver_position = Vec2::new(transform.translation.x, transform.translation.y);
            if let Some(direction) = (cursor_position.0 - diver_position).try_normalize() {
                let shape = Mesh::from(Circle::new(SPEAR_SIZE));
                let color = ColorMaterial::from(Color::rgb(1., 0., 0.));
                let mesh_handle = meshes.add(shape);
                let material_handle = materials.add(color);
                let spawn_position = diver_position + SPEAR_FIRE_RADIUS * direction;
                commands.spawn((
                    ProjectileBundle::new(
                        SPEAR_DAMAGE,
                        SPEAR_SIZE,
                        SPEAR_SIZE,
                        direction.x * SPEAR_INITIAL_VELOCITY + velocity.0.x,
                        direction.y * SPEAR_INITIAL_VELOCITY + velocity.0.y,
                    ),
                    MaterialMesh2dBundle {
                        mesh: mesh_handle.into(),
                        material: material_handle,
                        transform: Transform::from_translation(Vec3::new(
                            spawn_position.x,
                            spawn_position.y,
                            0.,
                        )),
                        ..default()
                    },
                ));
            }
        }
    }
}

#[test]
fn did_fire_speargun() {
    let mut app = App::new();
    app.add_systems(Update, fire_speargun);

    app.world
        .spawn((DiverBundle::new(), Transform::from_translation(Vec3::ZERO)));

    app.insert_resource(CursorPosition(Vec2::ONE));
    let mut mouse = ButtonInput::<MouseButton>::default();
    mouse.press(MouseButton::Left);
    app.insert_resource(mouse);

    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<ColorMaterial>::default());

    app.update();
    // should be one projectile
    assert_eq!(app.world.query::<&Projectile>().iter(&app.world).len(), 1);
    let (velocity, _) = app
        .world
        .query::<(&Velocity, &Projectile)>()
        .single(&app.world);
    // should be traveling at a 45deg angle
    assert_eq!(
        velocity.0,
        SPEAR_INITIAL_VELOCITY * Vec3::new(1., 1., 0.).normalize(),
    );

    app.world.resource_mut::<ButtonInput<MouseButton>>().clear();
    app.update();
    // should still be one projectile
    assert_eq!(app.world.query::<&Projectile>().iter(&app.world).len(), 1);
}

pub fn player_inhale(
    buttons: Res<ButtonInput<KeyCode>>,
    diver: Query<Entity, With<Diver>>,
    mut breaths: EventWriter<BreathTaken>,
) {
    if let Ok(diver_entity) = diver.get_single() {
        if buttons.just_pressed(KeyCode::Space) {
            breaths.send(BreathTaken {
                entity: diver_entity,
            });
        }
    }
}

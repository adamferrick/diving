use crate::animation::*;
use crate::bag::*;
use crate::collision::*;
use crate::drag::Drag;
use crate::equipment::EquippedCylinderJumpEvent;
use crate::health::*;
use crate::position::*;
use crate::projectile::*;
use crate::respiration::inhalation::*;
use crate::states::*;
use crate::BreatherBundle;
use crate::CursorPosition;
use crate::Spritesheets;
use bevy::prelude::*;

const DIVER_SPEED: f32 = 0.5;
const DIVER_DRAG: f32 = 0.9;
const DIVER_WIDTH: f32 = 5.;
const DIVER_HEIGHT: f32 = 13.;
const DIVER_ANIMATION_SPEED: f32 = 0.5;

const SPEAR_SIZE: f32 = 5.;
const SPEAR_INITIAL_VELOCITY: f32 = 1.5;
const SPEAR_DAMAGE: f32 = 40.;
const SPEAR_FIRE_RADIUS: f32 = 10.;

const DIVER_INITIAL_AMMO: u32 = 3;
const DIVER_INITIAL_BAG_SPACE: usize = 4;

pub const DIVER_TANK_CAPACITY: f32 = 1000.;
const DIVER_TANK_AMOUNT_REMAINING: f32 = 800.;
const DIVER_TANK_OXYGEN: f32 = 0.21;
const DIVER_TANK_NITROGEN: f32 = 0.78;

const DIVER_BLOODSTREAM_CAPACITY: f32 = 100.;
const DIVER_BLOODSTREAM_AMOUNT_REMAINING: f32 = 50.;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Diver;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EquippedAmmo(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Swimming(pub Vec3);

#[derive(Bundle)]
pub struct DiverBundle {
    diver: Diver,
    hitbox: RectangularHitbox,
    health: Health,
    velocity: Velocity,
    drag: Drag,
    equipped_tank: EquippedTank,
    equipped_ammo: EquippedAmmo,
    swimming: Swimming,
    breather_bundle: BreatherBundle,
}

impl DiverBundle {
    fn new(tank: Entity, ammo: Entity) -> Self {
        Self {
            diver: Diver,
            hitbox: RectangularHitbox(Rectangle::new(DIVER_WIDTH, DIVER_HEIGHT)),
            health: Health(100.),
            velocity: Velocity(Vec3::new(0., 0., 0.)),
            drag: Drag(DIVER_DRAG),
            equipped_tank: EquippedTank(tank),
            equipped_ammo: EquippedAmmo(ammo),
            swimming: Swimming(Vec3::ZERO),
            breather_bundle: BreatherBundle {
                bloodstream_content: BloodstreamContent {
                    capacity: DIVER_BLOODSTREAM_CAPACITY,
                    amount_remaining: DIVER_BLOODSTREAM_AMOUNT_REMAINING,
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn diver_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_diver.after(crate::load_assets));
    app.add_systems(
        Update,
        (
            player_control_swim.before(set_velocity_of_swimmer),
            fire_speargun
                .before(fire_projectile)
                .after(update_position)
                .after(crate::update_cursor),
            player_inhale.before(inhalation).after(update_position),
            player_gather,
            player_jump_cylinder,
        )
            .in_set(RunningStateSet)
            .in_set(NoMenuStateSet),
    );
    app.add_systems(FixedUpdate, set_velocity_of_swimmer.before(update_position));
    app.register_type::<Diver>();
    app.register_type::<EquippedAmmo>();
    app.register_type::<Swimming>();
}

pub fn spawn_diver(mut commands: Commands, spritesheets: Res<Spritesheets>) {
    println!("Spawning diver...");

    let cylinder_id = commands
        .spawn((
            DivingCylinder {
                capacity: DIVER_TANK_CAPACITY,
                amount_remaining: DIVER_TANK_AMOUNT_REMAINING,
                proportion_of_oxygen: DIVER_TANK_OXYGEN,
                proportion_of_nitrogen: DIVER_TANK_NITROGEN,
            },
            Collectible,
            Name::new("21%O2 78%N tank"),
        ))
        .id();

    let ammo_id = commands
        .spawn((
            Collectible,
            Ammo::Finite(DIVER_INITIAL_AMMO),
            Name::new("Ammo"),
        ))
        .id();

    let (texture, layout) = spritesheets.0.get("diver.png").unwrap();
    let animation_indices = AnimationIndices { first: 0, last: 1 };
    let diver_id = commands
        .spawn((
            DiverBundle::new(cylinder_id, ammo_id),
            Bag {
                collectibles: vec![cylinder_id, ammo_id],
                capacity: DIVER_INITIAL_BAG_SPACE,
            },
            SpriteBundle {
                transform: Transform::from_translation(Vec3::ZERO),
                texture: texture.clone(),
                ..default()
            },
            Name::new("Diver"),
            TextureAtlas {
                layout: layout.clone(),
                index: animation_indices.first,
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(
                DIVER_ANIMATION_SPEED,
                TimerMode::Repeating,
            )),
            crate::PIXEL_PERFECT_LAYERS,
        ))
        .id();
    commands.entity(ammo_id).insert(Collected(diver_id));
    commands.entity(cylinder_id).insert(Collected(diver_id));
}

pub fn player_control_swim(
    buttons: Res<ButtonInput<KeyCode>>,
    mut diver: Query<&mut Swimming, With<Diver>>,
) {
    if let Ok(mut swimming) = diver.get_single_mut() {
        let vertical = if buttons.pressed(KeyCode::ArrowUp) {
            Vec3::new(0., DIVER_SPEED, 0.)
        } else if buttons.pressed(KeyCode::ArrowDown) {
            Vec3::new(0., -DIVER_SPEED, 0.)
        } else {
            Vec3::ZERO
        };

        let horizontal = if buttons.pressed(KeyCode::ArrowLeft) {
            Vec3::new(-DIVER_SPEED, 0., 0.)
        } else if buttons.pressed(KeyCode::ArrowRight) {
            Vec3::new(DIVER_SPEED, 0., 0.)
        } else {
            Vec3::ZERO
        };

        *swimming = Swimming((horizontal + vertical).normalize_or_zero() * DIVER_SPEED);
    }
}

pub fn set_velocity_of_swimmer(mut swimmers: Query<(&mut Velocity, &Swimming)>) {
    for (mut velocity, swimming) in &mut swimmers {
        if swimming.0 != Vec3::ZERO {
            velocity.0 = swimming.0;
        }
    }
}

#[test]
fn did_set_velocity() {
    let mut app = App::new();
    app.add_systems(Update, set_velocity_of_swimmer);
    let swimmer_id = app
        .world_mut()
        .spawn((Velocity(Vec3::ZERO), Swimming(Vec3::new(1., 1., 0.))))
        .id();
    app.update();
    let new_velocity = app.world().get::<Velocity>(swimmer_id).unwrap();
    assert_eq!(new_velocity.0, Vec3::new(1., 1., 0.));
}

#[test]
fn did_not_set_velocity() {
    let mut app = App::new();
    app.add_systems(Update, set_velocity_of_swimmer);
    let swimmer_id = app
        .world_mut()
        .spawn((Velocity(Vec3::new(1., 1., 0.)), Swimming(Vec3::ZERO)))
        .id();
    app.update();
    let new_velocity = app.world().get::<Velocity>(swimmer_id).unwrap();
    assert_eq!(new_velocity.0, Vec3::new(1., 1., 0.));
}

pub fn fire_speargun(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    diver: Query<(&Transform, &Velocity, &EquippedAmmo), With<Diver>>,
    mut fire_events: EventWriter<FireProjectile>,
) {
    if let Ok((transform, velocity, ammo)) = diver.get_single() {
        if buttons.just_pressed(MouseButton::Left) {
            let diver_position = Vec2::new(transform.translation.x, transform.translation.y);
            if let Some(direction) = (cursor_position.0 - diver_position).try_normalize() {
                let spawn_position = diver_position + SPEAR_FIRE_RADIUS * direction;
                fire_events.send(FireProjectile {
                    translation: Vec3::new(spawn_position.x, spawn_position.y, 0.),
                    velocity: Vec3::new(
                        direction.x * SPEAR_INITIAL_VELOCITY + velocity.0.x,
                        direction.y * SPEAR_INITIAL_VELOCITY + velocity.0.y,
                        0.,
                    ),
                    dims: Rectangle::new(SPEAR_SIZE, SPEAR_SIZE),
                    damage: SPEAR_DAMAGE,
                    ammo: ammo.0,
                });
            }
        }
    }
}

#[test]
fn did_fire_speargun() {
    let mut app = App::new();
    app.add_systems(Update, fire_speargun);
    app.add_event::<FireProjectile>();

    let ammo_id = app.world_mut().spawn(Ammo::Infinite).id();

    app.world_mut().spawn((
        Diver,
        Velocity(Vec3::ZERO),
        Transform::from_translation(Vec3::ZERO),
        EquippedAmmo(ammo_id),
    ));

    app.insert_resource(CursorPosition(Vec2::ONE));
    let mut mouse = ButtonInput::<MouseButton>::default();
    mouse.press(MouseButton::Left);
    app.insert_resource(mouse);

    app.update();
    // should have sent an event
    let speargun_fire_events = app.world().resource::<Events<FireProjectile>>();
    let reader = speargun_fire_events.get_reader();
    assert!(!reader.is_empty(speargun_fire_events));

    app.world_mut()
        .resource_mut::<ButtonInput<MouseButton>>()
        .clear();
    app.update();
    // should not have sent an event
    let speargun_fire_events = app.world().resource::<Events<FireProjectile>>();
    assert_eq!(speargun_fire_events.len(), 1);
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

pub fn player_gather(
    mut commands: Commands,
    buttons: Res<ButtonInput<KeyCode>>,
    diver: Query<Entity, With<Diver>>,
) {
    if let Ok(diver_entity) = diver.get_single() {
        if let Some(mut entity_commands) = commands.get_entity(diver_entity) {
            if buttons.pressed(KeyCode::KeyG) {
                entity_commands.insert(Gathering);
            } else {
                entity_commands.remove::<Gathering>();
            }
        }
    }
}

pub fn player_jump_cylinder(
    buttons: Res<ButtonInput<KeyCode>>,
    diver: Query<Entity, With<Diver>>,
    mut cylinder_jumps: EventWriter<EquippedCylinderJumpEvent>,
) {
    if let Ok(diver_entity) = diver.get_single() {
        if buttons.just_pressed(KeyCode::KeyM) {
            cylinder_jumps.send(EquippedCylinderJumpEvent {
                i: 1,
                wearer: diver_entity,
            });
        } else if buttons.just_pressed(KeyCode::KeyN) {
            cylinder_jumps.send(EquippedCylinderJumpEvent {
                i: -1,
                wearer: diver_entity,
            });
        }
    }
}

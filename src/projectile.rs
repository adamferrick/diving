use crate::collision::*;
use crate::drag::Drag;
use crate::health::*;
use crate::position::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const PROJECTILE_DRAG: f32 = 0.99;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub enum Ammo {
    Infinite,
    Finite(u32),
}

#[derive(Event)]
pub struct FireProjectile {
    pub translation: Vec3,
    pub velocity: Vec3,
    pub dims: Rectangle,
    pub damage: f32,
    pub ammo: Entity,
}

#[derive(Event)]
pub struct ProjectileHit {
    pub projectile: Entity,
    pub target: Entity,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    damage: Damage,
    hitbox: RectangularHitbox,
    projectile: Projectile,
    velocity: Velocity,
    drag: Drag,
}

impl ProjectileBundle {
    pub fn new(damage: f32, dims: Rectangle, velocity: Vec3) -> Self {
        Self {
            damage: Damage(damage),
            hitbox: RectangularHitbox(dims),
            projectile: Projectile,
            velocity: Velocity(velocity),
            drag: Drag(PROJECTILE_DRAG),
        }
    }
}

pub fn projectile_plugin(app: &mut App) {
    app.add_event::<FireProjectile>();
    app.add_event::<ProjectileHit>();
    app.add_systems(
        FixedUpdate,
        (projectile_hit.after(projectile_collision), fire_projectile),
    );
}

pub fn fire_projectile(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ammos: Query<&mut Ammo>,
    mut fire_events: EventReader<FireProjectile>,
) {
    for fire_event in fire_events.read() {
        let shape = Mesh::from(fire_event.dims);
        let color = ColorMaterial::from(Color::rgb(1., 0., 0.));
        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);
        if let Ok(ammo) = ammos.get_mut(fire_event.ammo) {
            if let Ammo::Finite(ammo_left) = ammo.into_inner() {
                if *ammo_left == 0 {
                    return;
                } else {
                    *ammo_left -= 1;
                }
            }
            println!(
                "firing projectile. position: {}, velocity: {}",
                fire_event.translation, fire_event.velocity
            );
            commands.spawn((
                ProjectileBundle::new(fire_event.damage, fire_event.dims, fire_event.velocity),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform::from_translation(fire_event.translation),
                    ..default()
                },
            ));
        }
    }
}

#[test]
fn did_fire_projectile() {
    let mut app = App::new();
    app.add_event::<FireProjectile>();
    app.add_systems(Update, fire_projectile);
    app.world.insert_resource(Assets::<Mesh>::default());
    app.world
        .insert_resource(Assets::<ColorMaterial>::default());
    let ammo_id = app.world.spawn(Ammo::Infinite).id();
    app.world
        .resource_mut::<Events<FireProjectile>>()
        .send(FireProjectile {
            translation: Vec3::ZERO,
            velocity: Vec3::ONE,
            dims: Rectangle::new(1., 1.),
            damage: 1.,
            ammo: ammo_id,
        });
    app.update();
    // should be one projectile
    assert!(app
        .world
        .query::<&Projectile>()
        .get_single(&app.world)
        .is_ok());
    let (damage, hitbox, velocity, transform, _) = app
        .world
        .query::<(
            &Damage,
            &RectangularHitbox,
            &Velocity,
            &Transform,
            &Projectile,
        )>()
        .single(&app.world);
    // should have the values sent
    assert_eq!(damage.0, 1.);
    assert_eq!(hitbox.0, Rectangle::new(1., 1.));
    assert_eq!(velocity.0, Vec3::ONE);
    assert_eq!(transform.translation, Vec3::ZERO);
}

#[test]
fn did_fire_projectile_finite() {
    let mut app = App::new();
    app.add_event::<FireProjectile>();
    app.add_systems(Update, fire_projectile);
    app.world.insert_resource(Assets::<Mesh>::default());
    app.world
        .insert_resource(Assets::<ColorMaterial>::default());
    let ammo_id = app.world.spawn(Ammo::Finite(2)).id();
    app.world
        .resource_mut::<Events<FireProjectile>>()
        .send(FireProjectile {
            translation: Vec3::ZERO,
            velocity: Vec3::ONE,
            dims: Rectangle::new(1., 1.),
            damage: 1.,
            ammo: ammo_id,
        });
    app.update();
    // should be one projectile
    assert!(app
        .world
        .query::<&Projectile>()
        .get_single(&app.world)
        .is_ok());
    let (damage, hitbox, velocity, transform, _) = app
        .world
        .query::<(
            &Damage,
            &RectangularHitbox,
            &Velocity,
            &Transform,
            &Projectile,
        )>()
        .single(&app.world);
    // should have the values sent
    assert_eq!(damage.0, 1.);
    assert_eq!(hitbox.0, Rectangle::new(1., 1.));
    assert_eq!(velocity.0, Vec3::ONE);
    assert_eq!(transform.translation, Vec3::ZERO);
    // should have reduced ammo
    if let Ammo::Finite(ammo) = app.world.get::<Ammo>(ammo_id).unwrap() {
        assert_eq!(*ammo, 1);
    } else {
        panic!();
    }
}

#[test]
fn did_not_fire_projectile_empty() {
    let mut app = App::new();
    app.add_event::<FireProjectile>();
    app.add_systems(Update, fire_projectile);
    app.world.insert_resource(Assets::<Mesh>::default());
    app.world
        .insert_resource(Assets::<ColorMaterial>::default());
    let ammo_id = app.world.spawn(Ammo::Finite(0)).id();
    app.world
        .resource_mut::<Events<FireProjectile>>()
        .send(FireProjectile {
            translation: Vec3::ZERO,
            velocity: Vec3::ONE,
            dims: Rectangle::new(1., 1.),
            damage: 1.,
            ammo: ammo_id,
        });
    app.update();
    // should be one projectile
    assert!(app
        .world
        .query::<&Projectile>()
        .get_single(&app.world)
        .is_err());
}

pub fn projectile_hit(
    mut commands: Commands,
    projectiles: Query<&Damage, With<Projectile>>,
    targets: Query<Entity, (With<Health>, Without<Dead>)>,
    mut hit_events: EventReader<ProjectileHit>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for hit_event in hit_events.read() {
        if let (Ok(damage), Ok(target)) = (
            projectiles.get(hit_event.projectile),
            targets.get(hit_event.target),
        ) {
            damage_events.send(DamageEvent {
                target: target,
                damage: damage.0,
            });
            commands.entity(hit_event.projectile).despawn();
        }
    }
}

#[test]
fn hit_target() {
    let mut app = App::new();
    app.add_event::<ProjectileHit>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, projectile_hit);
    const DAMAGE: f32 = 5.;
    let projectile_id = app.world.spawn((Projectile, Damage(DAMAGE))).id();
    let target_id = app.world.spawn((Health(10.),)).id();
    // Send hit event
    app.world
        .resource_mut::<Events<ProjectileHit>>()
        .send(ProjectileHit {
            projectile: projectile_id,
            target: target_id,
        });
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next().unwrap();
    // Should have sent a DamageEvent { target: target_id, damage: DAMAGE }
    assert_eq!(damage.target, target_id);
    assert_eq!(damage.damage, DAMAGE);
    // Projectile should have despawned
    assert_eq!(app.world.query::<&Projectile>().iter(&app.world).len(), 0);
}

#[test]
fn do_not_hit_dead_target() {
    let mut app = App::new();
    app.add_event::<ProjectileHit>();
    app.add_event::<DamageEvent>();
    app.add_systems(Update, projectile_hit);
    const DAMAGE: f32 = 5.;
    let projectile_id = app.world.spawn((Projectile, Damage(DAMAGE))).id();
    let target_id = app.world.spawn((Health(0.), Dead)).id();
    // Send hit event
    app.world
        .resource_mut::<Events<ProjectileHit>>()
        .send(ProjectileHit {
            projectile: projectile_id,
            target: target_id,
        });
    app.update();
    let damage_events = app.world.resource::<Events<DamageEvent>>();
    let mut damage_reader = damage_events.get_reader();
    let damage = damage_reader.read(damage_events).next();
    // Should be no damage events
    assert!(damage.is_none());
    // Projectile should not have despawned
    assert_eq!(app.world.query::<&Projectile>().iter(&app.world).len(), 1);
}

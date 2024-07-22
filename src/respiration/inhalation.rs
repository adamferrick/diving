use crate::circulation::CirculateGas;
use crate::states::RunningStateSet;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const AIR_O2_RATIO: f32 = 0.21;
const AIR_N2_RATIO: f32 = 0.78;

const CYLINDER_WIDTH: f32 = 10.;
const CYLINDER_HEIGHT: f32 = 20.;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DivingCylinder {
    pub capacity: f32,
    pub amount_remaining: f32,
    pub proportion_of_oxygen: f32,
    pub proportion_of_nitrogen: f32,
}

impl Default for DivingCylinder {
    fn default() -> Self {
        Self {
            capacity: 0.,
            amount_remaining: 0.,
            proportion_of_oxygen: AIR_O2_RATIO,
            proportion_of_nitrogen: AIR_N2_RATIO,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EquippedTank(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct BloodstreamContent {
    pub capacity: f32,
    pub amount_remaining: f32,
    pub proportion_of_oxygen: f32,
    pub proportion_of_nitrogen: f32,
}

impl Default for BloodstreamContent {
    fn default() -> Self {
        Self {
            capacity: 0.,
            amount_remaining: 0.,
            proportion_of_oxygen: AIR_O2_RATIO,
            proportion_of_nitrogen: AIR_N2_RATIO,
        }
    }
}

#[derive(Event)]
pub struct BreathTaken {
    pub entity: Entity,
}

pub fn inhalation_plugin(app: &mut App) {
    app.add_event::<BreathTaken>();
    app.add_systems(Startup, spawn_cylinders);
    app.add_systems(FixedUpdate, inhalation.in_set(RunningStateSet));
    app.register_type::<DivingCylinder>();
    app.register_type::<BloodstreamContent>();
    app.register_type::<EquippedTank>();
}

pub fn spawn_cylinders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning cylinders");

    let mut spawn_cylinder =
        |x: f32, y: f32, proportion_of_oxygen: f32, proportion_of_nitrogen: f32| {
            let mesh = Mesh::from(Rectangle::new(CYLINDER_WIDTH, CYLINDER_HEIGHT));
            let material = ColorMaterial::from_color(Srgba::rgb(0.5, 0.5, 0.5));

            let mesh_handle = meshes.add(mesh);
            let material_handle = materials.add(material);

            commands.spawn((
                DivingCylinder {
                    capacity: crate::diver::DIVER_TANK_CAPACITY,
                    amount_remaining: crate::diver::DIVER_TANK_CAPACITY,
                    proportion_of_oxygen: proportion_of_oxygen,
                    proportion_of_nitrogen: proportion_of_nitrogen,
                },
                crate::collision::RectangularHitbox(Rectangle::new(CYLINDER_WIDTH, CYLINDER_HEIGHT)),
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                    ..default()
                },
                crate::bag::Collectible,
                Name::new(format!(
                    "{}O2 {}N tank",
                    proportion_of_oxygen, proportion_of_nitrogen
                )),
            ));
        };

    spawn_cylinder(100., 0., 0.5, 0.5);
    spawn_cylinder(200., 0., 0.3, 0.7);
}

pub fn inhalation(
    mut breathers: Query<(Entity, &mut BloodstreamContent, &EquippedTank)>,
    mut cylinders: Query<&mut DivingCylinder>,
    mut breaths: EventReader<BreathTaken>,
    mut circulate_gas: EventWriter<CirculateGas>,
) {
    for breath in breaths.read() {
        if let Ok((entity, bloodstream_content, equipped_tank_id)) =
            breathers.get_mut(breath.entity)
        {
            if let Ok(mut cylinder) = cylinders.get_mut(equipped_tank_id.0) {
                let amount_breathed = (bloodstream_content.capacity
                    - bloodstream_content.amount_remaining)
                    .min(cylinder.amount_remaining);
                cylinder.amount_remaining -= amount_breathed;
                println!(
                    "amount breathed: {}, tank remaining: {}",
                    amount_breathed, cylinder.amount_remaining,
                );
                if amount_breathed > 0. {
                    circulate_gas.send(CirculateGas {
                        entity: entity,
                        amount: amount_breathed,
                        proportion_of_oxygen: cylinder.proportion_of_oxygen,
                        proportion_of_nitrogen: cylinder.proportion_of_nitrogen,
                    });
                }
            }
        }
    }
}

#[test]
fn did_inhale_full() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world_mut()
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 100.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.5,
        })
        .id();
    let breather_id = app
        .world_mut()
        .spawn((
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 50.,
                proportion_of_oxygen: 0.,
                proportion_of_nitrogen: 0.,
            },
            EquippedTank(cylinder_id),
        ))
        .id();
    app.world_mut()
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    // cylinder proportion should be half empty
    let new_cylinder = app.world().get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 50.);
    // should have sent an event
    let gas_to_circulate_events = app.world().resource::<Events<CirculateGas>>();
    let mut gas_to_circulate_reader = gas_to_circulate_events.get_reader();
    let gas_to_circulate = gas_to_circulate_reader
        .read(gas_to_circulate_events)
        .next()
        .unwrap();
    assert_eq!(gas_to_circulate.entity, breather_id);
    assert_eq!(gas_to_circulate.amount, 50.);
    assert_eq!(gas_to_circulate.proportion_of_oxygen, 0.5);
}

#[test]
fn did_inhale_partial() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world_mut()
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 50.,
            proportion_of_oxygen: 0.5,
            proportion_of_nitrogen: 0.5,
        })
        .id();
    let breather_id = app
        .world_mut()
        .spawn((
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 25.,
                proportion_of_oxygen: 0.0,
                proportion_of_nitrogen: 0.0,
            },
            EquippedTank(cylinder_id),
        ))
        .id();
    app.world_mut()
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    // cylinder proportion should be empty
    let new_cylinder = app.world().get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 0.);
    // should have sent an event
    let gas_to_circulate_events = app.world().resource::<Events<CirculateGas>>();
    let mut gas_to_circulate_reader = gas_to_circulate_events.get_reader();
    let gas_to_circulate = gas_to_circulate_reader
        .read(gas_to_circulate_events)
        .next()
        .unwrap();
    assert_eq!(gas_to_circulate.entity, breather_id);
    assert_eq!(gas_to_circulate.amount, 50.);
    assert_eq!(gas_to_circulate.proportion_of_oxygen, 0.5);
    assert_eq!(gas_to_circulate.proportion_of_nitrogen, 0.5);
}

#[test]
fn did_not_inhale_empty_cylinder() {
    let mut app = App::new();
    app.add_event::<BreathTaken>();
    app.add_event::<CirculateGas>();
    app.add_systems(Update, inhalation);
    let cylinder_id = app
        .world_mut()
        .spawn(DivingCylinder {
            capacity: 100.,
            amount_remaining: 0.,
            proportion_of_oxygen: 0.,
            proportion_of_nitrogen: 0.,
        })
        .id();
    let breather_id = app
        .world_mut()
        .spawn((
            BloodstreamContent {
                capacity: 100.,
                amount_remaining: 50.,
                proportion_of_oxygen: 0.5,
                proportion_of_nitrogen: 0.5,
            },
            EquippedTank(cylinder_id),
        ))
        .id();
    app.world_mut()
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    // cylinder proportion still should be empty
    let new_cylinder = app.world().get::<DivingCylinder>(cylinder_id).unwrap();
    assert_eq!(new_cylinder.amount_remaining, 0.);
    // should not have sent an event
    let gas_to_circulate_events = app.world().resource::<Events<CirculateGas>>();
    let mut gas_to_circulate_reader = gas_to_circulate_events.get_reader();
    let gas_to_circulate = gas_to_circulate_reader.read(gas_to_circulate_events).next();
    assert!(gas_to_circulate.is_none());
}

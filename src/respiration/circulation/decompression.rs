use crate::inhalation::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct InertGasInBloodstream(pub f32);

#[derive(Component)]
pub struct SafeOutgassingRate(pub f32);

pub fn absorbing_and_outgassing(
    mut breathers: Query<(&Transform, &SafeOutgassingRate, &mut InertGasInBloodstream)>,
    mut breaths: EventReader<BreathTaken>,
) {
    for breath in breaths.read() {
        if let Ok((transform, safe_outgassing_rate, mut inert_gas_in_bloodstream)) =
            breathers.get_mut(breath.entity)
        {
            let depth = transform.translation.y.abs();
            if depth < inert_gas_in_bloodstream.0 {
                inert_gas_in_bloodstream.0 = depth;
            } else {
                inert_gas_in_bloodstream.0 =
                    (inert_gas_in_bloodstream.0 + safe_outgassing_rate.0).max(depth);
            }
        }
    }
}

#[test]
fn absorb() {
    let mut app = App::new();
    app.add_systems(Update, absorbing_and_outgassing);
    app.add_event::<BreathTaken>();
    let breather_id = app
        .world
        .spawn((
            Transform::from_translation(Vec3::new(0., -100., 0.)),
            InertGasInBloodstream(0.),
            SafeOutgassingRate(0.),
        ))
        .id();
    app.world
        .resource_mut::<Events<BreathTaken>>()
        .send(BreathTaken {
            entity: breather_id,
        });
    app.update();
    let inert_gas = app
        .world
        .get::<InertGasInBloodstream>(breather_id)
        .unwrap()
        .0;
    assert_eq!(inert_gas, 100.);
}

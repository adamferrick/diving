use bevy::prelude::*;

use crate::position::Velocity;
use crate::states::PausedState;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Drag(pub f32);

pub fn drag_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        apply_drag.run_if(in_state(PausedState::Running)),
    );
    app.register_type::<Drag>();
}

pub fn apply_drag(mut movables: Query<(&Drag, &mut Velocity)>) {
    for (drag, mut velocity) in &mut movables {
        velocity.0 *= drag.0;
    }
}

#[test]
fn did_drag() {
    let mut app = App::new();
    app.add_systems(Update, apply_drag);
    let movable_id = app
        .world_mut()
        .spawn((Drag(0.5), Velocity(Vec3::new(1., 1., 0.))))
        .id();
    app.update();
    let movable_velocity = app.world().get::<Velocity>(movable_id).unwrap();
    assert_eq!(movable_velocity.0, Vec3::new(0.5, 0.5, 0.));
}

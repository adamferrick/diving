use bevy::prelude::*;

#[derive(Component)]
pub struct EquippedTank {
    pub capacity: f32,
    pub proportion_remaining: f32,
    pub proportion_of_oxygen: f32,
}

#[derive(Component)]
pub struct Lungs {
    pub capacity: f32,
    pub proportion_remaining: f32,
}

#[derive(Event)]
pub struct BreathTaken {
    pub entity: Entity,
}

pub fn inhalation(
    mut breathers: Query<(&mut EquippedTank, &mut Lungs)>,
    mut breaths: EventReader<BreathTaken>,
) {
    for breath in breaths.read() {
        if let Ok((mut tank, mut lungs)) = breathers.get_mut(breath.entity) {
            let amount_breathed = (lungs.capacity * (1.0 - lungs.proportion_remaining))
                .min(tank.capacity * tank.proportion_remaining);
            println!("amount breathed: {}", amount_breathed);
        }
    }
}

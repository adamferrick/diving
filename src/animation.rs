use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer);

pub fn animation_plugin(app: &mut App) {
    app.add_systems(Update, animate_sprite);
    app.register_type::<AnimationIndices>();
    app.register_type::<AnimationTimer>();
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

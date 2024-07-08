use bevy::prelude::*;

#[derive(Component)]
pub struct Collectible;

#[derive(Component)]
pub struct Collected(pub Entity);

use bevy::prelude::*;

use std::collections::HashMap;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Spritesheets(pub HashMap<String, (Handle<Image>, Handle<TextureAtlasLayout>)>);

pub fn loading_plugin(app: &mut App) {
    app.init_resource::<Spritesheets>();
    app.register_type::<Spritesheets>();
    app.add_systems(Startup, load_assets);
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut spritesheets: ResMut<Spritesheets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("diver.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(6, 14), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    spritesheets
        .0
        .insert(String::from("diver.png"), (texture, texture_atlas_layout));
}

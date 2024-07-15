use crate::InGameMenuState;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InventoryMenu;

pub fn inventory_menu_plugin(app: &mut App) {
    app.register_type::<InventoryMenu>();
    app.add_systems(OnEnter(InGameMenuState::Inventory), spawn_inventory_menu);
    app.add_systems(OnExit(InGameMenuState::Inventory), despawn_inventory_menu);
}

pub fn spawn_inventory_menu(mut commands: Commands) {
    let container = NodeBundle {
        style: Style {
            width: Val::Percent(50.),
            height: Val::Percent(50.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Srgba::rgb(0., 0., 1.).into(),
        ..default()
    };
    commands.spawn((container, InventoryMenu, Name::new("Inventory menu")));
}

pub fn despawn_inventory_menu(
    mut commands: Commands,
    inventory_menus: Query<Entity, With<InventoryMenu>>,
) {
    if let Ok(inventory_menu) = inventory_menus.get_single() {
        commands.entity(inventory_menu).despawn_recursive();
    }
}

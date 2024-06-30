use crate::diver::*;
use crate::health::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HealthText;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_health_ui);
    app.add_systems(FixedUpdate, update_health_ui.after(damage_health));
}

pub fn spawn_health_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(10.),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("Ui Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: 32.,
                            ..default()
                        },
                    ),
                    ..default()
                },
                HealthText,
            ));
        });
}

pub fn update_health_ui(
    mut texts: Query<&mut Text, With<HealthText>>,
    health_query: Query<&Health, With<Diver>>,
) {
    for mut text in &mut texts {
        if let Ok(health) = health_query.get_single() {
            text.sections[0].value = format!("Health: {}", health.0);
        }
    }
}

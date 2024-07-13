use crate::diver::*;
use crate::health::*;
use crate::respiration::inhalation::*;
use bevy::prelude::*;

pub const FONT_SIZE: f32 = 32.;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HealthText;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CirculationText;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_health_ui);
    app.add_systems(
        FixedUpdate,
        (
            update_health_ui.after(damage_health),
            update_respiration_ui.after(inhalation),
        ),
    );
}

pub fn spawn_health_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(10.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: Srgba::rgb(0., 0., 1.).into(),
                ..default()
            },
            Name::new("Ui Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                HealthText,
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
            commands.spawn((
                CirculationText,
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                    ),
                    ..default()
                },
            ));
        });
}

pub fn update_health_ui(
    mut texts: Query<&mut Text, With<HealthText>>,
    health_query: Query<&Health, With<Diver>>,
) {
    for mut text in &mut texts {
        if let Ok(health) = health_query.get_single() {
            text.sections[0].value = format!("Health: {0:.0}", health.0);
        }
    }
}

pub fn update_respiration_ui(
    mut texts: Query<&mut Text, With<CirculationText>>,
    diver_query: Query<(&BloodstreamContent, &EquippedTank), With<Diver>>,
    cylinder_query: Query<&DivingCylinder>,
) {
    for mut text in &mut texts {
        if let Ok((bloodstream, equipped_tank)) = diver_query.get_single() {
            if let Ok(cylinder) = cylinder_query.get(equipped_tank.0) {
                text.sections[0].value = format!(
                    "Breath remaining: {0:.0}%, Tank remaining: {1:.0}%",
                    (bloodstream.amount_remaining / bloodstream.capacity) * 100.,
                    (cylinder.amount_remaining / cylinder.capacity) * 100.,
                );
            }
        }
    }
}

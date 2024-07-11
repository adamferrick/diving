use crate::ui::FONT_SIZE;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Paused,
    Running,
}

#[derive(Component)]
pub struct PauseMenu;

pub fn states_plugin(app: &mut App) {
    app.init_state::<PausedState>();
    app.add_systems(Update, toggle_pause);
    app.add_systems(OnEnter(PausedState::Paused), spawn_paused_message);
    app.add_systems(OnExit(PausedState::Paused), despawn_paused_message);
}

pub fn toggle_pause(
    paused_state: Res<State<PausedState>>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match paused_state.get() {
            PausedState::Paused => next_paused_state.set(PausedState::Running),
            PausedState::Running => next_paused_state.set(PausedState::Paused),
        }
    }
}

pub fn spawn_paused_message(mut commands: Commands) {
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
        background_color: Color::BLUE.into(),
        ..default()
    };
    let message = TextBundle {
        text: Text::from_section(
            "Game paused...",
            TextStyle {
                font_size: FONT_SIZE,
                ..default()
            },
        ),
        ..default()
    };

    let container_id = commands.spawn((container, PauseMenu)).id();
    let message_id = commands.spawn(message).id();
    commands.entity(container_id).push_children(&[message_id]);
}

pub fn despawn_paused_message(mut commands: Commands, pause_menus: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu) = pause_menus.get_single() {
        commands.entity(pause_menu).despawn_recursive();
    }
}

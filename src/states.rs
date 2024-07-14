use crate::ui::FONT_SIZE;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Paused,
    Running,
    OpenInventory,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunningStateSet;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PauseMenu;

pub fn states_plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_systems(Update, toggle_pause);
    app.add_systems(OnEnter(GameState::Paused), spawn_paused_message);
    app.add_systems(OnExit(GameState::Paused), despawn_paused_message);
    app.configure_sets(
        FixedUpdate,
        RunningStateSet.run_if(in_state(GameState::Running)),
    );
    app.register_type::<PauseMenu>();
}

pub fn toggle_pause(
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::Paused => next_game_state.set(GameState::Running),
            _ => next_game_state.set(GameState::Paused),
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
        background_color: Srgba::rgb(0., 0., 1.).into(),
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

    let container_id = commands
        .spawn((container, PauseMenu, Name::new("Pause menu")))
        .id();
    let message_id = commands.spawn(message).id();
    commands.entity(container_id).push_children(&[message_id]);
}

pub fn despawn_paused_message(mut commands: Commands, pause_menus: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu) = pause_menus.get_single() {
        commands.entity(pause_menu).despawn_recursive();
    }
}

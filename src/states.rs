use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Paused,
    Running,
}

pub fn states_plugin(app: &mut App) {
    app.init_state::<PausedState>();
    app.add_systems(Update, toggle_pause);
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

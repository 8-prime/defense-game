use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .add_sub_state::<PhaseState>();
    app.add_systems(Update, handle_escape.run_if(in_state(GameState::Playing)));
}

#[derive(Resource, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Playing)]
pub enum InGameState {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Playing)]
pub enum PhaseState {
    #[default]
    Preparing,
    Active,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}

#[derive(Resource)]
pub struct LevelState {
    pub index: u64,
    pub timer: Timer,
}

pub fn handle_escape(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(match current_state.get() {
            InGameState::Running => InGameState::Paused,
            InGameState::Paused => InGameState::Running,
        });
    }
}

use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>().add_sub_state::<InGameState>();
    app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);

    app.add_systems(OnEnter(GameState::GameOver), setup_game_over_menu);

    app.add_systems(Update, handle_start_button);
    app.add_systems(Update, handle_main_menu_button);
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

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct MainMenuButton;

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(GameState::MainMenu),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    StartGameButton,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    DespawnOnExit(GameState::Playing),
                ))
                .with_child((Text::new("Start Game"), TextFont::from_font_size(24.0)));
        });
}

pub fn setup_game_over_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(GameState::GameOver),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    StartGameButton,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((Text::new("Restart Game"), TextFont::from_font_size(24.0)));
            parent
                .spawn((
                    Button,
                    MainMenuButton,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((Text::new("Main Menu"), TextFont::from_font_size(24.0)));
        });
}

pub fn handle_start_button(
    buttons: Query<Has<Pressed>, With<StartGameButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for button in buttons {
        if button {
            game_state.set(GameState::Playing);
        }
    }
}

pub fn handle_main_menu_button(
    buttons: Query<Has<Pressed>, With<MainMenuButton>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for button in buttons {
        if button {
            game_state.set(GameState::MainMenu);
        }
    }
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

use bevy::{
    app::{App, Update},
    camera::Camera3d,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Has, With},
        resource::Resource,
        system::{Commands, Query, ResMut},
    },
    math::Vec3,
    state::{
        app::AppExtStates,
        state::{NextState, OnEnter, OnExit, States},
    },
    text::TextFont,
    time::Timer,
    transform::components::Transform,
    ui::{AlignItems, JustifyContent, Node, Pressed, Val, widget::Text},
    ui_widgets::Button,
    utils::default,
};

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
    app.add_systems(OnExit(GameState::MainMenu), tear_down_main_menu);
    app.add_systems(Update, handle_start_button);
}

#[derive(Resource, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
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

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(-10.081, 20.0, 23.668))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
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
                .with_child((Text::new("Start Game"), TextFont::from_font_size(24.0)));
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

pub fn tear_down_main_menu(
    mut commands: Commands,
    start_game_buttons: Query<Entity, With<StartGameButton>>,
) {
    for start_button in start_game_buttons {
        commands.entity(start_button).despawn();
    }
}

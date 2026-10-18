use bevy::{
    camera::Camera3d,
    ecs::{
        component::Component,
        entity::Entity,
        query::{Has, With},
        resource::Resource,
        system::{Commands, Query, ResMut},
    },
    math::Vec3,
    state::state::{NextState, States},
    text::{Justify, TextFont, TextLayout, Underline},
    time::Timer,
    transform::components::Transform,
    ui::{
        Node, PositionType, Pressed, Val, px,
        widget::{Text, TextShadow},
    },
    ui_widgets::Button,
    utils::default,
};

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

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands
        .spawn((
            Button,
            StartGameButton,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(60.0),
                ..default()
            },
        ))
        .with_child((Text::new("Start Game"), TextFont::from_font_size(24.0)));
}

pub fn tear_down_main_menu(
    mut commands: Commands,
    start_game_buttons: Query<Entity, With<StartGameButton>>,
) {
    for start_button in start_game_buttons {
        commands.entity(start_button).despawn();
    }
}

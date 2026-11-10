use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};

use crate::{menu::start_menu::StartGameButton, state::GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), setup_game_over_menu);
    app.add_systems(Update, handle_main_menu_button);
}

#[derive(Component)]
pub struct MainMenuButton;

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

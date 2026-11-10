use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};

use crate::state::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
    app.add_systems(Update, handle_start_button);
}

#[derive(Component)]
pub struct StartGameButton;

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

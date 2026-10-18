use bevy::prelude::*;

use crate::{
    damage::{DamageMessage, apply_damage, handle_deaths},
    enemy::show_health,
    levels::one::spawn_level,
    player::handle_player_attack,
    state::{GameState, handle_start_button, setup_main_menu, tear_down_main_menu},
};

mod base;
mod damage;
mod enemy;
mod levels;
mod player;
mod state;

pub struct DefensegamePlugin;

impl Plugin for DefensegamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu);
        app.add_systems(OnExit(GameState::MainMenu), tear_down_main_menu);
        app.add_systems(OnEnter(GameState::Playing), spawn_level);
        app.add_message::<DamageMessage>();
        // Disabled for now and spawned on init
        // app.add_systems(Update, spawn_enemies.run_if(resource_exists::<LevelState>));
        app.add_systems(Update, handle_start_button);
        app.add_systems(
            Update,
            (
                handle_player_attack,
                apply_damage,
                handle_deaths,
                show_health,
            )
                .chain(),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefensegamePlugin)
        .run();
}

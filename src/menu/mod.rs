use bevy::prelude::*;

mod game_over_menu;
mod start_menu;

pub fn plugin(app: &mut App) {
    game_over_menu::plugin(app);
    start_menu::plugin(app);
}

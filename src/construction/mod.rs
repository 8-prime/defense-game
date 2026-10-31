use bevy::prelude::*;

mod placement;
mod ui;

pub fn plugin(app: &mut App) {
    ui::plugin(app);
}

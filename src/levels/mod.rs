use bevy::{app::App, ecs::resource::Resource};

use crate::levels::phase::Phase;

pub mod one;
pub mod path;
pub mod phase;

pub fn plugin(app: &mut App) {
    one::plugin(app);
    phase::plugin(app);
}


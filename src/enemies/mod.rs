use bevy::prelude::*;

pub mod slime;

pub fn plugin(app: &mut App) {
    slime::plugin(app);
}

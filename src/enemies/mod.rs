use bevy::prelude::*;

mod components;
pub mod slime;

pub fn plugin(app: &mut App) {
    slime::plugin(app);
}

#[derive(Component)]
pub struct Enemy {
    pub damage: u64,
}

use bevy::prelude::*;

mod bullet;
mod camera;
mod construction;
mod core;
mod damage;
mod debug;
mod enemy;
mod generator;
mod hitbox;
mod levels;
mod paths;
mod player;
mod state;
mod towers;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        levels::plugin,
        damage::plugin,
        enemy::plugin,
        player::plugin,
        bullet::plugin,
        state::plugin,
        towers::plugin,
        construction::plugin,
        camera::plugin,
        debug::plugin,
        generator::plugin,
    ));
}

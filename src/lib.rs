use bevy::prelude::*;

mod bullet;
mod camera;
mod construction;
mod core;
mod currency;
mod damage;
mod debug;
mod enemies;
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
        enemies::plugin,
        player::plugin,
        bullet::plugin,
        state::plugin,
        towers::plugin,
        construction::plugin,
        camera::plugin,
        debug::plugin,
        generator::plugin,
        currency::plugin,
    ));
}

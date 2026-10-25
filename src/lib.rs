use bevy::prelude::*;

mod bullet;
mod camera;
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
mod turret;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        levels::plugin,
        damage::plugin,
        enemy::plugin,
        player::plugin,
        bullet::plugin,
        state::plugin,
        turret::plugin,
        camera::plugin,
        debug::plugin,
        generator::plugin,
    ));
}

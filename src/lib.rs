use bevy::prelude::*;

mod base;
mod camera;
mod damage;
mod enemy;
mod levels;
mod player;
mod shoot;
mod state;
mod turret;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        levels::plugin,
        damage::plugin,
        enemy::plugin,
        player::plugin,
        shoot::plugin,
        state::plugin,
        turret::plugin,
        camera::plugin,
    ));
}

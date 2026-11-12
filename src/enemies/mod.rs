use bevy::prelude::*;

use crate::enemies::slime::{SlimeColor, SlimeMaterials, spawn_slime};

mod components;
pub mod slime;

pub fn plugin(app: &mut App) {
    slime::plugin(app);
    app.add_message::<SpawnEnemyMessage>();
}

#[derive(Message)]
pub struct SpawnEnemyMessage {
    pub enemy_type: EnemyType,
    pub path_index: usize,
}

#[derive(Component)]
pub struct Enemy {
    pub damage: u64,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    Slime(SlimeColor),
}

pub fn spawn_enemy(
    enemy_type: EnemyType,
    commands: &mut Commands,
    asset_server: &AssetServer,
    transform: Transform,
    materials: &SlimeMaterials,
    path: usize,
    waypoint: usize,
) {
    match enemy_type {
        EnemyType::Slime(slime_color) => {
            spawn_slime(
                commands,
                asset_server,
                transform,
                materials,
                slime_color,
                path,
                waypoint,
            );
        }
    }
}

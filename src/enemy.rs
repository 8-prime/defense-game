use bevy::{
    app::{App, Update},
    asset::AssetServer,
    color::Color,
    ecs::{
        component::Component,
        hierarchy::Children,
        query::{Changed, With},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    gltf::GltfAssetLabel,
    math::{Quat, Vec3},
    sprite::Text2d,
    state::condition::in_state,
    text::{TextColor, TextFont},
    time::Time,
    transform::components::Transform,
    world_serialization::WorldAssetRoot,
};
// use bevy::prelude::*;

use crate::{
    core::MovementSpeed,
    damage::{DamageMessage, Health, handle_deaths},
    hitbox::on_enemy_loaded,
    paths::{PathFollower, Paths},
    state::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_message::<DamageMessage>();
    // apply_damage
    app.add_systems(
        Update,
        (move_enemy, show_health, scale_enemies)
            .chain()
            .run_if(in_state(GameState::Playing))
            .after(handle_deaths),
    );
}

#[derive(Component)]
pub struct Enemy {
    pub damage: u64,
}

pub fn scale_enemies(enemies: Query<(&Health, &mut Transform), (With<Enemy>, Changed<Health>)>) {
    for (health, mut transform) in enemies {
        let size_ratio = (health.0 / 100.0).clamp(0.0, 1.0);

        transform.scale = Vec3::splat(0.5 + 0.5 * size_ratio);
    }
}

pub fn show_health(
    enemies: Query<(&Health, &Children), (With<Enemy>, Changed<Health>)>,
    mut texts: Query<&mut Text2d>,
) {
    for (health, children) in &enemies {
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(*child) {
                **text = format!("{}", health.0);
            }
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    transform: Transform,
    path: usize,
    waypoint: usize,
) {
    commands
        .spawn((
            Enemy { damage: 1 },
            Health(100.0),
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemy.glb"))),
            PathFollower { path, waypoint },
            MovementSpeed(2.0),
            transform,
        ))
        .with_child((
            Text2d::new("100"),
            TextFont::from_font_size(24.0),
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 0.0, 3.5)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .observe(on_enemy_loaded);
}

pub fn move_enemy(
    time: Res<Time>,
    enemies: Query<(&mut PathFollower, &mut Transform, &MovementSpeed), With<Enemy>>,
    paths: Res<Paths>,
) {
    for (mut path_follower, mut transform, movement_speed) in enemies {
        let Some(path) = paths.paths.get(path_follower.path) else {
            continue;
        };

        let Some(target) = path.get(path_follower.waypoint) else {
            continue;
        };

        let distance = movement_speed.0 * time.delta_secs();
        transform.translation = transform.translation.move_towards(*target, distance);

        if transform.translation.distance_squared(*target) < 0.0001 {
            path_follower.waypoint += 1;
        }
    }
}

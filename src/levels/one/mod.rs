use bevy::prelude::*;

use crate::{
    enemies::{
        SpawnEnemyMessage,
        slime::{SlimeColor, SlimeMaterials, spawn_slime},
    },
    generator::spawn_generator,
    levels::path::enemy_path,
    paths::Paths,
    player::spawn_player,
    state::GameState,
};

mod phases;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level);
    app.add_systems(OnEnter(GameState::Playing), setup_enemy_resources);
    app.add_systems(
        Update,
        spawn_enemies.chain().run_if(in_state(GameState::Playing)),
    );
}

pub fn setup_enemy_resources(mut commands: Commands) {
    commands.insert_resource(Paths {
        paths: vec![enemy_path()],
    });
}

pub fn spawn_enemies(
    mut messages: MessageReader<SpawnEnemyMessage>,
    mut commands: Commands,
    materials: Res<SlimeMaterials>,
    enemy_paths: Res<Paths>,
    asset_server: Res<AssetServer>,
) {
    for message in messages.read() {
        let Some(path) = enemy_paths.paths.get(message.path_index) else {
            return;
        };

        let Some(start) = path.get(0) else {
            return;
        };

        spawn_slime(
            &mut commands,
            &asset_server,
            Transform::from_translation(*start),
            &materials,
            SlimeColor::Green,
            0,
            1,
        );
    }
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Single<Entity, With<Camera3d>>,
) {
    // Floor
    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("floor.glb"))),
        DespawnOnExit(GameState::Playing),
    ));

    spawn_generator(&mut commands, &asset_server);
    spawn_player(&mut commands, &asset_server);

    // Lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(GameState::Playing),
    ));
}

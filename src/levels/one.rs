use bevy::prelude::*;

use crate::{
    enemies::slime::spawn_enemy, generator::spawn_generator, levels::path::enemy_path,
    paths::Paths, player::spawn_player, state::GameState,
};

#[derive(Resource)]
pub struct SpawnTarget(pub u64);

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level);
    app.add_systems(Update, spawn_enemies.run_if(in_state(GameState::Playing)));
    app.add_systems(OnEnter(GameState::Playing), setup_enemy_resources);
}

pub fn setup_enemy_resources(mut commands: Commands) {
    commands.insert_resource(Paths {
        paths: vec![enemy_path()],
    });

    commands.insert_resource(SpawnTarget(50));
}

pub fn spawn_enemies(
    time: Res<Time>,
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut spawn_target: ResMut<SpawnTarget>,
    enemy_paths: Res<Paths>,
    asset_server: Res<AssetServer>,
) {
    if spawn_target.0 == 0 {
        return;
    }

    let Some(path) = enemy_paths.paths.get(0) else {
        return;
    };

    let Some(start) = path.get(0) else {
        return;
    };

    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.is_finished() {
        return;
    }
    spawn_timer.0.reset();

    spawn_enemy(
        &mut commands,
        &asset_server,
        Transform::from_translation(*start),
        0,
        1,
    );
    spawn_target.0 -= 1;
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera: Single<Entity, With<Camera3d>>,
) {
    commands.insert_resource(EnemySpawnTimer::default());
    // Floor
    commands.spawn((WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("floor.glb")),
    ),));

    spawn_generator(&mut commands, &asset_server);
    spawn_player(&mut commands, &asset_server);

    // Lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

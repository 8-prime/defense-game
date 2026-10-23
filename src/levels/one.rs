use bevy::prelude::*;

use crate::{
    enemy::spawn_enemy, paths::Paths, player::spawn_player, state::GameState, turret::spawn_turret,
};

#[derive(Component)]
pub struct Generator;

#[derive(Component)]
pub struct GeneratorParts {
    pub stabalizer: Entity,
}

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
    app.add_systems(Update, resolve_generator_parts);
    app.add_systems(Update, animate_generator);
    app.add_systems(Update, spawn_enemies.run_if(in_state(GameState::Playing)));
    app.add_systems(OnEnter(GameState::Playing), setup_enemy_resources);
}

pub fn resolve_generator_parts(
    mut commands: Commands,
    generators: Query<Entity, (With<Generator>, Without<GeneratorParts>)>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for generator in generators {
        let mut stabalizer = None;

        for entity in children.iter_descendants(generator) {
            let Ok(name) = names.get(entity) else {
                continue;
            };

            match name.as_str() {
                "Stabalizer" => stabalizer = Some(entity),
                _ => {}
            }
        }

        let Some(stabalizer) = stabalizer else {
            continue;
        };

        commands
            .entity(generator)
            .insert(GeneratorParts { stabalizer });
    }
}

pub fn animate_generator(
    time: Res<Time>,
    generators: Query<&GeneratorParts, With<Generator>>,
    globals: Query<&GlobalTransform>,
    mut transforms: Query<&mut Transform>,
) {
    let angle = 90.0_f32.to_radians() * time.delta_secs();

    for parts in &generators {
        let Ok(mut transform) = transforms.get_mut(parts.stabalizer) else {
            continue;
        };

        transform.rotate_y(angle);
    }
}

pub fn setup_enemy_resources(mut commands: Commands) {
    commands.insert_resource(Paths {
        paths: vec![vec![
            Vec3::new(-10.0, 0.0, -10.0),
            Vec3::new(10.0, 0.0, -10.0),
            Vec3::new(10.0, 0.0, 10.0),
            Vec3::new(-10.0, 0.0, 10.0),
            Vec3::new(0.0, 0.0, 0.0),
        ]],
    });

    commands.insert_resource(SpawnTarget(5));
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(EnemySpawnTimer::default());
    // Floor
    commands.spawn((WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("floor.glb")),
    ),));

    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("power_core.glb"))),
        Generator {},
    ));

    // // 10 enemies in a row
    // for i in 0..10 {
    //     let x = (i as f32 - 4.5) * 1.5;

    //     spawn_enemy(
    //         &mut commands,
    //         &asset_server,
    //         Transform::from_xyz(x, 0.4, -3.0),
    //     );
    // }

    spawn_player(&mut commands, &asset_server);

    spawn_turret(
        &mut commands,
        &asset_server,
        Vec3 {
            x: 3.0,
            y: 0.4,
            z: 4.0,
        },
        0.8,
    );

    // Lighting
    commands.spawn((PointLight::default(), Transform::from_xyz(0.0, 10.0, 0.0)));
}

use bevy::prelude::*;

use crate::{enemy::spawn_enemy, player::spawn_player, state::GameState, turret::spawn_turret};

#[derive(Component)]
pub struct Generator;

#[derive(Component)]
pub struct GeneratorParts {
    pub stabalizer: Entity,
}

#[derive(Component)]
pub struct EnemyPath;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level);
    app.add_systems(Update, resolve_generator_parts);
    app.add_systems(Update, animate_generator);
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

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Floor
    commands.spawn((WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("floor.glb")),
    ),));

    commands.spawn((
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("power_core.glb"))),
        Generator {},
    ));

    // 10 enemies in a row
    for i in 0..10 {
        let x = (i as f32 - 4.5) * 1.5;

        spawn_enemy(
            &mut commands,
            &asset_server,
            Transform::from_xyz(x, 0.4, -3.0),
        );
    }

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

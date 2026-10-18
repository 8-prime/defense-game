use bevy::prelude::*;

use crate::{
    damage::Health,
    enemy::Enemy,
    player::{AttackCooldown, AttackRange, MovementSpeed, Player},
};

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 15.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.25, 0.2))),
    ));

    // Shared cube mesh
    let cube = meshes.add(Cuboid::new(0.8, 0.8, 0.8));

    let enemy_material = materials.add(Color::srgb(0.9, 0.2, 0.2));
    let player_material = materials.add(Color::srgb(0.2, 0.5, 1.0));

    // 10 enemies in a row
    for i in 0..10 {
        let x = (i as f32 - 4.5) * 1.5;

        commands
            .spawn((
                Enemy,
                Health(100.0),
                Mesh3d(cube.clone()),
                MeshMaterial3d(enemy_material.clone()),
                Transform::from_xyz(x, 0.4, -3.0),
            ))
            .with_child((
                Text2d::new("100"),
                TextFont::from_font_size(24.0),
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 0.0, 3.5)
                    .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ));
    }

    // Player below the enemies
    let mut timer = Timer::from_seconds(0.5, TimerMode::Once);
    timer.finish();

    commands.spawn((
        Player,
        Mesh3d(cube),
        MeshMaterial3d(player_material),
        Transform::from_xyz(0.0, 0.4, 4.0),
        AttackRange { range: 100.0 },
        AttackCooldown(timer),
        MovementSpeed(2.0),
    ));

    // Lighting
    commands.spawn((PointLight::default(), Transform::from_xyz(0.0, 10.0, 0.0)));
}

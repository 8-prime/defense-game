use bevy::prelude::*;

use crate::{base::AttackCooldown, damage::Health, enemy::Enemy, shoot::spawn_bullet_from_source};

#[derive(Component)]
pub struct Turret;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, handle_turret_attack);
}

pub fn handle_turret_attack(
    time: Res<Time>,
    enemies: Query<(Entity, &Transform, &Health), With<Enemy>>,
    turrets: Query<(&Transform, &mut AttackCooldown), With<Turret>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (turret_pos, mut cooldown) in turrets {
        cooldown.0.tick(time.delta());
        if !cooldown.0.is_finished() {
            return;
        }

        let Some((_, target_pos, _)) = enemies
            .iter()
            .min_by(|(_, _, a), (_, _, b)| a.0.total_cmp(&b.0))
        else {
            continue;
        };

        cooldown.0.reset();
        spawn_bullet_from_source(
            &mut commands,
            &mut meshes,
            &mut materials,
            turret_pos.clone(),
            target_pos.clone(),
            10.0, // speed
            33.0, // damage
            0.15, // size
        );
    }
}

pub fn spawn_turret(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
    size: f32,
) {
    commands.spawn((
        Turret,
        AttackCooldown(Timer::from_seconds(1.0, TimerMode::Repeating)),
        Mesh3d(meshes.add(Cylinder::new(size, size * 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.3, 1.0),
            metallic: 0.6,
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_translation(position),
    ));
}

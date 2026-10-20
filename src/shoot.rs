use bevy::{
    app::{App, Update},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        message::MessageWriter,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    math::Vec3,
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    shape::Sphere,
    state::condition::in_state,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{damage::DamageMessage, enemy::Enemy, state::GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (bullet_movement, bullet_damage)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

#[derive(Component)]
pub struct Bullet {
    pub dir: Vec3,
    pub speed: f32,
    pub damage: f32,
}

fn bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Transform, &Bullet)>,
) {
    for (e, mut transform, bullet) in &mut bullets.iter_mut() {
        transform.translation += bullet.dir * bullet.speed * time.delta_secs();
    }

    //check for out of screen bounds
}

fn bullet_damage(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &Bullet)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut message_writer: MessageWriter<DamageMessage>,
) {
    for (bullet_entity, transform, bullet) in bullets.iter() {
        for (enemy_entity, enemy_transform) in enemies {
            let distance_squared = transform
                .translation
                .distance_squared(enemy_transform.translation);

            if distance_squared < 0.5_f32.powi(2) {
                message_writer.write(DamageMessage {
                    target: enemy_entity,
                    amount: bullet.damage,
                });

                commands.entity(bullet_entity).despawn();
                break;
            }
        }
    }
}
pub fn spawn_bullet_from_source(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    source: Transform,
    target: Transform,
    speed: f32,
    damage: f32,
    size: f32,
) {
    let dir = (target.translation - source.translation).normalize_or_zero();

    spawn_bullet(
        commands,
        meshes,
        materials,
        dir,
        source.translation,
        speed,
        damage,
        size,
    );
}

pub fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    dir: Vec3,
    position: Vec3,
    speed: f32,
    damage: f32,
    size: f32,
) {
    commands.spawn((
        Bullet { dir, speed, damage },
        Mesh3d(meshes.add(Sphere::new(size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.75, 0.0),
            metallic: 0.8,
            perceptual_roughness: 0.25,
            ..default()
        })),
        Transform::from_translation(position),
    ));
}

use bevy::{
    app::{App, Update},
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        message::MessageWriter,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    gltf::GltfAssetLabel,
    math::Vec3,
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    shape::Sphere,
    state::condition::in_state,
    time::Time,
    transform::components::Transform,
    utils::default,
    world_serialization::WorldAssetRoot,
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

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &AssetServer,
    dir: Vec3,
    position: Vec3,
    speed: f32,
    damage: f32,
    size: f32,
) {
    commands.spawn((
        Bullet { dir, speed, damage },
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("bullet.glb"))),
        Transform::from_translation(position).with_scale(Vec3::splat(2.0)),
    ));
}

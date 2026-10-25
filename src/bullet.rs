use bevy::{
    app::{App, Update},
    asset::AssetServer,
    camera::primitives::Aabb,
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
    shape::IntersectsVolume,
    state::condition::in_state,
    time::Time,
    transform::components::{GlobalTransform, Transform},
    world_serialization::WorldAssetRoot,
};

use crate::{
    damage::DamageMessage,
    enemy::Enemy,
    hitbox::{Hitbox, on_enemy_loaded, to_aabb2d},
    state::GameState,
};

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
    bullets: Query<(Entity, &Bullet, &Hitbox)>,
    enemies: Query<(Entity, &Hitbox), With<Enemy>>,
    hitboxes: Query<(&Aabb, &GlobalTransform)>,
    mut message_writer: MessageWriter<DamageMessage>,
) {
    for (bullet_entity, bullet, bullet_hitbox) in &bullets {
        let Ok((bullet_aabb, bullet_transform)) = hitboxes.get(bullet_hitbox.0) else {
            continue;
        };

        let bullet_bounds = to_aabb2d(bullet_aabb, bullet_transform);

        for (enemy_entity, enemy_hitbox) in &enemies {
            let Ok((enemy_aabb, enemy_transform)) = hitboxes.get(enemy_hitbox.0) else {
                continue;
            };

            let enemy_bounds = to_aabb2d(enemy_aabb, enemy_transform);

            if !bullet_bounds.intersects(&enemy_bounds) {
                continue;
            }

            message_writer.write(DamageMessage {
                target: enemy_entity,
                amount: bullet.damage,
            });

            commands.entity(bullet_entity).despawn();
            break;
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
    commands
        .spawn((
            Bullet { dir, speed, damage },
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("bullet.glb"))),
            Transform::from_translation(position).with_scale(Vec3::splat(2.0)),
        ))
        .observe(on_enemy_loaded);
}

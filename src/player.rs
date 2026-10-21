use bevy::prelude::*;

use crate::{
    base::{AttackCooldown, AttackRange},
    damage::DamageMessage,
    enemy::Enemy,
    state::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    app.add_systems(
        Update,
        handle_player_attack.run_if(in_state(GameState::Playing)),
    );
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

pub fn handle_player_attack(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut player: Single<(&Transform, &AttackRange, &mut AttackCooldown), With<Player>>,
    mut message_writer: MessageWriter<DamageMessage>,
) {
    let (position, range, cooldown) = &mut *player;
    cooldown.0.tick(time.delta());
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if !cooldown.0.is_finished() {
        return;
    }
    println!("Attacking {} enemies", enemies.iter().len());
    cooldown.0.reset();

    for (entity, enemy) in enemies {
        if !range.in_range(*position, enemy) {
            println!("Enemy {} out of range", entity);
            continue;
        }
        println!("Attacking {}", entity);
        message_writer.write(DamageMessage {
            amount: 33.0,
            target: entity,
        });
    }
}

pub fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Transform, &MovementSpeed), With<Player>>,
) {
    let (transform, speed) = &mut *player;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let dir = direction.normalize_or_zero();

    if dir == Vec3::ZERO {
        return;
    }

    transform.translation += dir * speed.0 * time.delta_secs();

    let target_rotation = Transform::IDENTITY.looking_to(dir, Vec3::Y).rotation;
    let max_angle = 360.0_f32.to_radians() * time.delta_secs();

    transform.rotation = transform
        .rotation
        .rotate_towards(target_rotation, max_angle);
}

pub fn spawn_player(commands: &mut Commands, asset_server: &AssetServer) {
    let mut timer = Timer::from_seconds(0.5, TimerMode::Once);
    timer.finish();
    commands.spawn((
        Player,
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("player.glb"))),
        Transform::from_xyz(0.0, 0.4, 4.0),
        AttackRange { range: 10.0 },
        AttackCooldown(timer),
        MovementSpeed(2.0),
    ));
}

use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        message::MessageWriter,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Query, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::Vec3,
    state::condition::in_state,
    time::{Time, Timer},
    transform::components::Transform,
};

use crate::{damage::DamageMessage, enemy::Enemy, state::GameState};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    app.add_systems(
        Update,
        handle_player_attack.run_if(in_state(GameState::Playing)),
    );
}

#[derive(Component)]
pub struct AttackRange {
    pub range: f32,
}

impl AttackRange {
    pub fn in_range(&self, source: &Transform, target: &Transform) -> bool {
        let dist = (source.translation - target.translation).length();
        // let dx = source.x - target.x;
        // let dz = source.z - target.z;
        // let distance_squared = dx * dx + dz * dz;

        // // println!(
        // //     "Source: {:?}, Target: {:?}, Distance²: {}, Range²: {}",
        // //     source,
        // //     target,
        // //     distance_squared,
        // //     self.range * self.range
        // // );
        return dist <= self.range;
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

#[derive(Component)]
pub struct AttackCooldown(pub Timer);

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
    transform.translation += direction.normalize_or_zero() * speed.0 * time.delta_secs();
}

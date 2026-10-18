use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        message::MessageWriter,
        query::With,
        system::{Query, Res, Single},
    },
    input::{ButtonInput, mouse::MouseButton},
    time::{Time, Timer},
};

use crate::{base::Position, damage::DamageMessage, enemy::Enemy};

#[derive(Component)]
pub struct AttackRange {
    pub range: f32,
}

impl AttackRange {
    pub fn in_range(&self, source: &Position, target: &Position) -> bool {
        let dx = source.x - target.x;
        let dz = source.z - target.z;
        let distance_squared = dx * dx + dz * dz;

        // println!(
        //     "Source: {:?}, Target: {:?}, Distance²: {}, Range²: {}",
        //     source,
        //     target,
        //     distance_squared,
        //     self.range * self.range
        // );
        distance_squared <= self.range * self.range
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AttackCooldown(pub Timer);

pub fn handle_player_attack(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    enemies: Query<(Entity, &Position), With<Enemy>>,
    mut player: Single<(&Position, &AttackRange, &mut AttackCooldown), With<Player>>,
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

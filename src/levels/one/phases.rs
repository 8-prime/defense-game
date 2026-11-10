use std::time::Duration;

use crate::{
    currency::PlayerCurrencies,
    enemies::{EnemyType, slime::SlimeColor},
    levels::phase::{EnemySpawnInfo, Phase, PhaseReward},
};

pub fn get_phase(index: usize) -> Option<Phase> {
    return match index {
        1 => Some(phase_one()),
        _ => None,
    };
}

pub fn phase_one() -> Phase {
    return Phase {
        enemies: vec![
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(1000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(1500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(2000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(2500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(3000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(3500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(4000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(4500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(5000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(5500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(6000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(6500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(7000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(7500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(8000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(8500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(9000),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(9500),
            },
            EnemySpawnInfo {
                enemy_type: EnemyType::Slime(SlimeColor::Green),
                path_index: 0,
                spawn_at: Duration::from_millis(10000),
            },
        ],
        reward: PhaseReward {
            rewards: PlayerCurrencies::from(20, 10, 20),
        },
    };
}

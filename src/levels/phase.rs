use bevy::prelude::*;
use std::time::Duration;

use crate::{
    currency::PlayerCurrencies,
    enemies::{Enemy, EnemyType, SpawnEnemyMessage},
    levels::one::spawn_level,
    state::PhaseState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (tick_phase, complete_phase)
            .chain()
            .after(spawn_level)
            .run_if(in_state(PhaseState::Active)),
    );
}

#[derive(Message)]
pub struct PhaseCompleteMessage;

pub struct Phase {
    pub enemies: Vec<EnemySpawnInfo>,
    pub reward: PhaseReward,
}

pub struct PhaseReward {
    pub rewards: PlayerCurrencies,
}

pub struct EnemySpawnInfo {
    pub enemy_type: EnemyType,
    pub path_index: usize,
    pub spawn_at: Duration,
}

#[derive(Resource)]
pub struct CurrentPhase {
    pub phase: Phase,
    pub spawn_list_cursor: usize,
    pub phase_timer: Timer,
    pub spawning_complete: bool,
}

pub fn tick_phase(
    time: Res<Time>,
    mut current_phase: ResMut<CurrentPhase>,
    mut message_writer: MessageWriter<SpawnEnemyMessage>,
) {
    current_phase.phase_timer.tick(time.delta());

    if current_phase.spawning_complete {
        return;
    }

    if current_phase.spawn_list_cursor == current_phase.phase.enemies.len() {
        current_phase.spawning_complete = true;
        return;
    }

    let mut enemy_messages = Vec::with_capacity(current_phase.phase.enemies.len());

    let mut cursor = current_phase.spawn_list_cursor;
    for (index, spawn_info) in current_phase.phase.enemies[current_phase.spawn_list_cursor..]
        .iter()
        .enumerate()
    {
        if spawn_info.spawn_at > current_phase.phase_timer.elapsed() {
            current_phase.spawn_list_cursor = index;
            break;
        }

        enemy_messages.push(SpawnEnemyMessage {
            enemy_type: spawn_info.enemy_type,
            path_index: spawn_info.path_index,
        });

        cursor += 1;
    }
    current_phase.spawn_list_cursor = cursor;
    message_writer.write_batch(enemy_messages);
}

pub fn complete_phase(
    mut message_writer: MessageWriter<PhaseCompleteMessage>,
    mut next_phase_state: ResMut<NextState<PhaseState>>,
    enemies: Query<&Enemy>,
    current_phase: Res<CurrentPhase>,
    mut currency: ResMut<PlayerCurrencies>,
) {
    if enemies.count() > 0 || !current_phase.spawning_complete {
        return;
    }
    next_phase_state.set(PhaseState::Preparing);
    message_writer.write(PhaseCompleteMessage);
    currency.deposit(&current_phase.phase.reward.rewards);
}

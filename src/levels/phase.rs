use bevy::{prelude::*, time::Stopwatch, ui::Pressed, ui_widgets::Button};
use std::time::Duration;

use crate::{
    currency::PlayerCurrencies,
    enemies::{Enemy, EnemyType, SpawnEnemyMessage},
    levels::one::spawn_level,
    state::PhaseState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(PhaseState::Preparing), phase_ui);
    app.add_systems(Update, handle_start_phase);
    app.add_systems(
        Update,
        (tick_phase, complete_phase)
            .chain()
            .after(spawn_level)
            .run_if(in_state(PhaseState::Active)),
    );
}

#[derive(Resource)]
pub struct LevelPhases {
    next_phase: usize,
    phases: Vec<Phase>,
}

#[derive(Clone)]
pub struct Phase {
    pub enemies: Vec<EnemySpawnInfo>,
    pub reward: PhaseReward,
}

#[derive(Clone)]
pub struct PhaseReward {
    pub rewards: PlayerCurrencies,
}

#[derive(Clone, Copy)]
pub struct EnemySpawnInfo {
    pub enemy_type: EnemyType,
    pub path_index: usize,
    pub spawn_at: Duration,
}

#[derive(Resource)]
pub struct CurrentPhase {
    pub phase: Phase,
    pub spawn_list_cursor: usize,
    pub phase_stopwatch: Stopwatch,
    pub spawning_complete: bool,
}

impl From<Phase> for CurrentPhase {
    fn from(value: Phase) -> Self {
        return CurrentPhase {
            phase: value,
            phase_stopwatch: Stopwatch::new(),
            spawn_list_cursor: 0,
            spawning_complete: false,
        };
    }
}

pub fn tick_phase(
    time: Res<Time>,
    mut current_phase: ResMut<CurrentPhase>,
    mut message_writer: MessageWriter<SpawnEnemyMessage>,
) {
    current_phase.phase_stopwatch.tick(time.delta());

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
        if spawn_info.spawn_at > current_phase.phase_stopwatch.elapsed() {
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

#[derive(Component)]
pub struct StartPhaseButton;

pub fn phase_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                top: px(0),
                width: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            DespawnOnExit(PhaseState::Preparing),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    StartPhaseButton,
                    Node {
                        width: Val::Px(60.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_child((Text::new("▶"), TextFont::from_font_size(24.0)));
        });
}

pub fn handle_start_phase(
    buttons: Query<Has<Pressed>, With<StartPhaseButton>>,
    mut game_state: ResMut<NextState<PhaseState>>,
    mut commands: Commands,
    mut phase_info: ResMut<LevelPhases>,
) {
    for button in buttons {
        if button {
            let Some(next_phase) = phase_info.phases.get(phase_info.next_phase) else {
                continue;
            };
            commands.insert_resource(CurrentPhase::from(next_phase.clone()));
            game_state.set(PhaseState::Active);
            phase_info.next_phase += 1;
        }
    }
}

pub fn complete_phase(
    mut next_phase_state: ResMut<NextState<PhaseState>>,
    enemies: Query<&Enemy>,
    current_phase: Res<CurrentPhase>,
    mut currency: ResMut<PlayerCurrencies>,
) {
    if enemies.count() > 0 || !current_phase.spawning_complete {
        return;
    }
    next_phase_state.set(PhaseState::Preparing);
    currency.deposit(&current_phase.phase.reward.rewards);
}

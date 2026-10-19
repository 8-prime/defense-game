use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        hierarchy::Children,
        query::{Changed, With},
        schedule::IntoScheduleConfigs,
        system::Query,
    },
    math::Vec3,
    sprite::Text2d,
    state::condition::in_state,
    transform::components::Transform,
};

use crate::{
    damage::{DamageMessage, Health, handle_deaths},
    state::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_message::<DamageMessage>();
    // apply_damage
    app.add_systems(
        Update,
        (show_health, scale_enemies)
            .chain()
            .run_if(in_state(GameState::Playing))
            .after(handle_deaths),
    );
}

#[derive(Component)]
pub struct Enemy;

pub fn scale_enemies(enemies: Query<(&Health, &mut Transform), (With<Enemy>, Changed<Health>)>) {
    for (health, mut transform) in enemies {
        let size_ratio = (health.0 / 100.0).clamp(0.0, 1.0);

        transform.scale = Vec3::splat(0.5 + 0.5 * size_ratio);
    }
}

pub fn show_health(
    enemies: Query<(&Health, &Children), (With<Enemy>, Changed<Health>)>,
    mut texts: Query<&mut Text2d>,
) {
    for (health, children) in &enemies {
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(*child) {
                **text = format!("{}", health.0);
            }
        }
    }
}

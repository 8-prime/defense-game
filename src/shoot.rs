use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        entity::Entity,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res},
    },
    math::Vec3,
    state::condition::in_state,
    time::Time,
    transform::components::Transform,
};

use crate::state::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, bullet_movement.run_if(in_state(GameState::Playing)));
}

#[derive(Component)]
pub struct Bullet {
    pub dir: Vec3,
    pub speed: f32,
    pub target: Entity,
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

pub fn spawn_bullet(
    commands: &mut Commands,
    dir: Vec3,
    position: Vec3,
    speed: f32,
    damage: f32,
    size: f32,
    glyph: usize,
) {
    // commands.spawn((
    // bullet component + transform + target entity
    // ));
}

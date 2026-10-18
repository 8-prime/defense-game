use bevy::ecs::{
    component::Component,
    entity::Entity,
    message::{Message, MessageReader},
    system::{Commands, Query},
};

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32,
}

pub fn apply_damage(mut messages: MessageReader<DamageMessage>, mut health: Query<&mut Health>) {
    for message in messages.read() {
        if let Ok(mut hp) = health.get_mut(message.target) {
            hp.0 -= message.amount;
        }
    }
}

pub fn handle_deaths(entities: Query<(Entity, &Health)>, mut commands: Commands) {
    for (entity, health) in &entities {
        if health.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::{
    ecs::{
        component::Component,
        hierarchy::Children,
        query::{Changed, With},
        system::Query,
    },
    sprite::Text2d,
};

use crate::{base::Position, damage::Health};

#[derive(Component)]
pub struct Enemy;
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

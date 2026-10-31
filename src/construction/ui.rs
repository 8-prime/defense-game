use bevy::prelude::*;
use bevy::{ecs::name::Name, picking::hover::Hovered, ui_widgets::Button};

use crate::state::GameState;
use crate::towers::{TowerAssets, tower_type::TowerType};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_construction_ui);
}

pub fn spawn_construction_ui(mut commands: Commands, tower_assets: Res<TowerAssets>) {
    commands
        .spawn((
            Name::new("Construction UI"),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(0),
                left: px(0),
                width: percent(100),
                height: px(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: px(12),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.07, 0.9)),
        ))
        .with_children(|parent| {
            for tower_type in TowerType::ALL {
                let Some(visuals) = &tower_assets.towers.get(&tower_type) else {
                    continue;
                };
                parent
                    .spawn((
                        Name::new(format!("{tower_type:?} Button")),
                        Button,
                        Hovered::default(),
                        Node {
                            width: px(72),
                            height: px(72),
                            padding: UiRect::all(px(6)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_child((
                        ImageNode::new(visuals.icon.clone()),
                        Node {
                            width: percent(100),
                            height: percent(100),
                            ..default()
                        },
                    ));
            }
        });
}

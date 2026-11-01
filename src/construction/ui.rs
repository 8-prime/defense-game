use bevy::prelude::*;
use bevy::ui::Pressed;
use bevy::{ecs::name::Name, picking::hover::Hovered, ui_widgets::Button};

use crate::construction::placement::{
    ActivePlacement, PlacementState, TowerPreview, begin_placement, stop_placement,
};
use crate::state::GameState;
use crate::towers::{TowerAssets, tower_type::TowerType};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_construction_ui);
    app.add_systems(Update, handle_tower_button);
}

#[derive(Component)]
pub struct TowerButton(TowerType);

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
                        TowerButton(tower_type),
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

pub fn handle_tower_button(
    buttons: Query<(Has<Pressed>, &TowerButton)>,
    mut placement_state: ResMut<PlacementState>,
    mut commands: Commands,
    assets: Res<TowerAssets>,
) {
    for (pressed, button) in &buttons {
        if !pressed {
            continue;
        }

        let tower_type = button.0;

        let already_selected = placement_state
            .active
            .as_ref()
            .is_some_and(|active| active.tower_type == tower_type);

        if already_selected {
            stop_placement(&mut commands, &mut placement_state);
        } else {
            begin_placement(&mut commands, &mut placement_state, &assets, tower_type);
        }
    }
}

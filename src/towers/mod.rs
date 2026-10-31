use std::collections::HashMap;

use bevy::prelude::*;

use crate::towers::tower_type::TowerType;

pub mod tower_type;
mod turret;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_tower_assets);
}

#[derive(Resource)]
pub struct TowerAssets {
    pub towers: HashMap<TowerType, TowerVisuals>,
}

pub struct TowerVisuals {
    pub scene: Handle<WorldAsset>,
    pub icon: Handle<Image>,
}

fn load_tower_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut towers = HashMap::new();

    for tower_type in TowerType::ALL {
        let definition = tower_type.define();
        towers.insert(
            tower_type,
            TowerVisuals {
                scene: asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset(definition.scene_path)),
                icon: asset_server.load(definition.icon_path),
            },
        );
    }
    commands.insert_resource(TowerAssets { towers });
}

pub fn spawn_tower(
    commands: &mut Commands,
    tower_type: TowerType,
    position: Vec3,
    tower_assets: &TowerAssets,
) {
    match tower_type {
        TowerType::Turret => turret::spawn_turret(commands, position, tower_assets),
    }
}

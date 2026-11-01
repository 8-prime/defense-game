use bevy::{prelude::*, window::PrimaryWindow};

use crate::towers::{TowerAssets, tower_type::TowerType};

pub fn plugin(app: &mut App) {
    app.insert_resource(PlacementState { active: None });
    app.add_systems(Update, (update_placement).chain());
}

#[derive(Resource)]
pub struct PlacementState {
    pub active: Option<ActivePlacement>,
}

pub struct ActivePlacement {
    pub tower_type: TowerType,
    pub preview_entity: Entity,
}

#[derive(Component)]
pub struct TowerPreview;

pub fn stop_placement(commands: &mut Commands, placement: &mut PlacementState) {
    if let Some(active) = placement.active.take() {
        commands.entity(active.preview_entity).despawn();
    }
}

pub fn begin_placement(
    commands: &mut Commands,
    placement: &mut PlacementState,
    assets: &TowerAssets,
    tower_type: TowerType,
) {
    if let Some(previous) = placement.active.take() {
        commands.entity(previous.preview_entity).despawn();
    }

    let Some(visuals) = &assets.towers.get(&tower_type) else {
        return;
    };

    let preview_entity = commands
        .spawn((
            TowerPreview,
            WorldAssetRoot(visuals.scene.clone()),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    placement.active = Some(ActivePlacement {
        tower_type,
        preview_entity,
    })
}

pub fn update_placement(
    placement: Res<PlacementState>,
    mut transforms: Query<&mut Transform, With<TowerPreview>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
) {
    let Some(active) = &placement.active else {
        return;
    };

    let Ok(mut transform) = transforms.get_mut(active.preview_entity) else {
        return;
    };

    let Some(cursor) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = *camera;

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
        return;
    };

    let Some(position) = ray.plane_intersection_point(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
    else {
        return;
    };

    transform.translation = position;
}

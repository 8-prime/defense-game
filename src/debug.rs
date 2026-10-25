use bevy::{camera::primitives::Aabb, prelude::*, window::PrimaryWindow};

use crate::hitbox::{Hitbox, to_aabb2d};

#[derive(Component)]
pub struct MouseCoordinates;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_mouse_coordinates);
    app.add_systems(Update, determine_coordinates);
    app.add_systems(Update, draw_hitboxes);
}

pub fn setup_mouse_coordinates(mut commands: Commands) {
    commands.spawn((
        MouseCoordinates,
        Text::new("World: ---"),
        TextFont::from_font_size(18.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(15),
            right: px(15),
            ..default()
        },
    ));
}

pub fn determine_coordinates(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut text: Single<&mut Text, With<MouseCoordinates>>,
) {
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

    **text = Text::new(format!(
        "X: {:.2} Y: {:.2} Z: {:.2}",
        position.x, position.y, position.z
    ))
}

pub fn draw_hitboxes(
    entities: Query<&Hitbox>,
    hitboxes: Query<(&Aabb, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for hitbox in &entities {
        let Ok((aabb, transform)) = hitboxes.get(hitbox.0) else {
            continue;
        };

        let bounds = to_aabb2d(aabb, transform);

        let min = bounds.min;
        let max = bounds.max;

        // Draw rectangles slightly above ground level
        let y = 0.1;

        gizmos.linestrip(
            [
                Vec3::new(min.x, y, min.y),
                Vec3::new(max.x, y, min.y),
                Vec3::new(max.x, y, max.y),
                Vec3::new(min.x, y, max.y),
                Vec3::new(min.x, y, min.y),
            ],
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}

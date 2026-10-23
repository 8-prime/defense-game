use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct MouseCoordinates;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_mouse_coordinates);
    app.add_systems(Update, determine_coordinates);
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

use bevy::prelude::*;

use crate::{
    player::{Player, move_player},
    state::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        follow_player
            .after(move_player)
            .run_if(in_state(GameState::Playing)),
    );
}

pub fn follow_player(
    time: Res<Time>,
    player: Query<&GlobalTransform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(player) = player.single() else {
        return;
    };

    let Ok(mut camera) = camera.single_mut() else {
        return;
    };

    let offset = Vec3::new(-7.0, 18.0, 17.0);
    let target_position = player.translation() + offset;

    // Higher = faster following
    let smoothing = 5.0;

    let t = 1.0 - (-smoothing * time.delta_secs()).exp();

    camera.translation = camera.translation.lerp(target_position, t);
}

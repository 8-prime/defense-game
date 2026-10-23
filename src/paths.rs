use bevy::prelude::*;

#[derive(Resource)]
pub struct Paths {
    pub paths: Vec<Vec<Vec3>>,
}

#[derive(Component)]
pub struct PathFollower {
    pub path: usize,
    pub waypoint: usize,
}

use bevy::{ecs::component::Component, time::Timer, transform::components::Transform};

#[derive(Component)]
pub struct AttackCooldown(pub Timer);

#[derive(Component)]
pub struct AttackRange {
    pub range: f32,
}

impl AttackRange {
    pub fn in_range(&self, source: &Transform, target: &Transform) -> bool {
        let dist = (source.translation - target.translation).length();
        // let dx = source.x - target.x;
        // let dz = source.z - target.z;
        // let distance_squared = dx * dx + dz * dz;

        // // println!(
        // //     "Source: {:?}, Target: {:?}, Distance²: {}, Range²: {}",
        // //     source,
        // //     target,
        // //     distance_squared,
        // //     self.range * self.range
        // // );
        return dist <= self.range;
    }
}

#[derive(Component)]
pub struct MovementSpeed(pub f32);

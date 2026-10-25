use bevy::{camera::primitives::Aabb, prelude::*, world_serialization::WorldInstanceReady};

#[derive(Component)]
pub struct Hitbox(pub Entity);

pub fn on_enemy_loaded(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    let hit_entity = ready.entity;

    for child in children.iter_descendants(hit_entity) {
        let Ok(name) = names.get(child) else {
            continue;
        };

        if name.as_str() == "Hitbox" {
            commands.entity(hit_entity).insert(Hitbox(child));

            commands.entity(child).insert(Visibility::Hidden);

            info!("Enemy hitbox found: {:?}", child);
            break;
        }
    }
}

pub fn to_aabb2d(aabb: &Aabb, global: &GlobalTransform) -> Aabb2d {
    let center = Vec3::from(aabb.center);
    let half = Vec3::from(aabb.half_extents);

    let mut min = Vec2::splat(f32::INFINITY);
    let mut max = Vec2::splat(f32::NEG_INFINITY);

    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
                let corner = center + half * Vec3::new(x, y, z);
                let world = global.transform_point(corner).xz();

                min = min.min(world);
                max = max.max(world);
            }
        }
    }

    Aabb2d { min, max }
}

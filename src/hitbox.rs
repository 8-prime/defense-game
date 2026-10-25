use bevy::{camera::primitives::Aabb, prelude::*, world_serialization::WorldInstanceReady};

#[derive(Component)]
pub struct Hitbox(pub Entity);

pub fn on_hittable_loaded(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    names: Query<&Name>,
    meshes: Query<(), With<Mesh3d>>,
) {
    let hit_entity = ready.entity;

    for node in children.iter_descendants(hit_entity) {
        let Ok(name) = names.get(node) else {
            continue;
        };

        if name.as_str() != "Hitbox" {
            continue;
        }

        // The mesh might be on the named node itself
        // or on one of its descendants.
        let mesh_entity = if meshes.contains(node) {
            Some(node)
        } else {
            children
                .iter_descendants(node)
                .find(|&entity| meshes.contains(entity))
        };

        let Some(mesh_entity) = mesh_entity else {
            warn!("Hitbox found, but no Mesh3d attached");
            continue;
        };

        commands.entity(hit_entity).insert(Hitbox(mesh_entity));

        // Hide the entire hitbox hierarchy
        commands.entity(node).insert(Visibility::Hidden);

        info!("Hitbox mesh found: {:?} (node: {:?})", mesh_entity, node);

        break;
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

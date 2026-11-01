use std::collections::HashMap;

use bevy::{
    app::{App, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::Color,
    ecs::{
        hierarchy::Children,
        query::{Changed, With},
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    gltf::GltfAssetLabel,
    math::{Quat, Vec3},
    pbr::{MeshMaterial3d, StandardMaterial, StandardMaterialUniform},
    reflect::map::map_partial_cmp,
    render::occlusion_culling,
    sprite::Text2d,
    state::condition::in_state,
    text::{TextColor, TextFont},
    time::Time,
    transform::components::Transform,
    utils::default,
    world_serialization::WorldAssetRoot,
};

use crate::{
    core::MovementSpeed,
    damage::{DamageMessage, Health, handle_deaths},
    enemies::Enemy,
    hitbox::on_hittable_loaded,
    paths::{PathFollower, Paths},
    state::GameState,
};

pub fn plugin(app: &mut App) {
    app.add_message::<DamageMessage>();
    // apply_damage
    app.add_systems(
        Update,
        (move_enemy, show_health, scale_enemies)
            .chain()
            .run_if(in_state(GameState::Playing))
            .after(handle_deaths),
    );
    app.add_systems(Startup, setup_materials);
}

#[derive(Resource)]
pub struct SlimeMaterials {
    pub materials: HashMap<SlimeColor, Handle<StandardMaterial>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum SlimeColor {
    Green,
    Red,
    Blue,
}

impl SlimeColor {
    pub const ALL: [Self; 3] = [Self::Blue, Self::Green, Self::Red];

    pub fn as_color(&self) -> Color {
        match self {
            SlimeColor::Green => Color::srgb(0.199999, 0.435301, 0.0),
            SlimeColor::Red => Color::srgb(0.0, 0.113507, 0.435312),
            SlimeColor::Blue => Color::srgb(0.435317, 0.024841, 0.004583),
        }
    }
}

pub fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut slime_materials = HashMap::new();

    for slime_color in SlimeColor::ALL {
        slime_materials.insert(
            slime_color,
            materials.add(StandardMaterial {
                base_color: slime_color.as_color(),
                metallic: 0.0,
                perceptual_roughness: 0.18,
                ior: 1.4,
                diffuse_transmission: 0.3,
                ..default()
            }),
        );
    }

    commands.insert_resource(SlimeMaterials {
        materials: slime_materials,
    });
}

pub fn scale_enemies(enemies: Query<(&Health, &mut Transform), (With<Enemy>, Changed<Health>)>) {
    for (health, mut transform) in enemies {
        let size_ratio = (health.0 / 100.0).clamp(0.0, 1.0);

        transform.scale = Vec3::splat(0.5 + 0.5 * size_ratio);
    }
}

pub fn show_health(
    enemies: Query<(&Health, &Children), (With<Enemy>, Changed<Health>)>,
    mut texts: Query<&mut Text2d>,
) {
    for (health, children) in &enemies {
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(*child) {
                **text = format!("{}", health.0);
            }
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    transform: Transform,
    materials: &SlimeMaterials,
    slime_color: SlimeColor,
    path: usize,
    waypoint: usize,
) {
    let Some(handle) = materials.materials.get(&slime_color) else {
        return;
    };

    commands
        .spawn((
            Enemy { damage: 1 },
            Health(100.0),
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("enemy.glb"))),
            MeshMaterial3d(handle.clone()),
            PathFollower { path, waypoint },
            MovementSpeed(2.0),
            transform,
        ))
        .with_child((
            Text2d::new("100"),
            TextFont::from_font_size(24.0),
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 0.0, 3.5)
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ))
        .observe(on_hittable_loaded);
}

pub fn move_enemy(
    time: Res<Time>,
    enemies: Query<(&mut PathFollower, &mut Transform, &MovementSpeed), With<Enemy>>,
    paths: Res<Paths>,
) {
    for (mut path_follower, mut transform, movement_speed) in enemies {
        let Some(path) = paths.paths.get(path_follower.path) else {
            continue;
        };

        let Some(target) = path.get(path_follower.waypoint) else {
            continue;
        };

        let distance = movement_speed.0 * time.delta_secs();
        transform.translation = transform.translation.move_towards(*target, distance);

        if transform.translation.distance_squared(*target) < 0.0001 {
            path_follower.waypoint += 1;
        }
    }
}

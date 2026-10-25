use bevy::{camera::primitives::Aabb, prelude::*, window::PrimaryWindow};

use crate::{
    enemy::Enemy,
    hitbox::{Hitbox, on_hittable_loaded, to_aabb2d},
    state::GameState,
};

#[derive(Component)]
pub struct Generator(u64);

#[derive(Component)]
pub struct GeneratorParts {
    pub stabalizer: Entity,
}

#[derive(Component)]
pub struct GeneratorHpDisplay(pub Entity);

pub fn plugin(app: &mut App) {
    app.add_systems(Update, resolve_generator_parts);
    app.add_systems(Update, animate_generator);
    app.add_systems(
        Update,
        (generator_damage, update_generator_hp)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

pub fn update_generator_hp(
    generators: Query<&Generator, Changed<Generator>>,
    mut displays: Query<(&GeneratorHpDisplay, &mut Text)>,
) {
    for (display, mut text) in &mut displays {
        let Ok(generator) = generators.get(display.0) else {
            continue;
        };

        text.0 = format!("Generator Health: {}", generator.0);
    }
}

fn generator_damage(
    mut commands: Commands,
    generators: Query<(&Hitbox, &mut Generator)>,
    enemies: Query<(Entity, &Hitbox, &Enemy)>,
    hitboxes: Query<(&Aabb, &GlobalTransform)>,
) {
    for (generator_hitbox, mut generator) in generators {
        let Ok((generator_aabb, generator_transform)) = hitboxes.get(generator_hitbox.0) else {
            continue;
        };

        let generator_bounds = to_aabb2d(generator_aabb, generator_transform);

        for (enemy_entity, enemy_hitbox, enemy) in &enemies {
            let Ok((enemy_aabb, enemy_transform)) = hitboxes.get(enemy_hitbox.0) else {
                continue;
            };

            let enemy_bounds = to_aabb2d(enemy_aabb, enemy_transform);

            if !generator_bounds.intersects(&enemy_bounds) {
                continue;
            }

            commands.entity(enemy_entity).despawn();
            generator.0 -= enemy.damage;
            break;
        }
    }
}

pub fn resolve_generator_parts(
    mut commands: Commands,
    generators: Query<Entity, (With<Generator>, Without<GeneratorParts>)>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for generator in generators {
        let mut stabalizer = None;

        for entity in children.iter_descendants(generator) {
            let Ok(name) = names.get(entity) else {
                continue;
            };

            match name.as_str() {
                "Stabalizer" => stabalizer = Some(entity),
                _ => {}
            }
        }

        let Some(stabalizer) = stabalizer else {
            continue;
        };

        commands
            .entity(generator)
            .insert(GeneratorParts { stabalizer });
    }
}

pub fn animate_generator(
    time: Res<Time>,
    generators: Query<&GeneratorParts, With<Generator>>,
    globals: Query<&GlobalTransform>,
    mut transforms: Query<&mut Transform>,
) {
    let angle = 90.0_f32.to_radians() * time.delta_secs();

    for parts in &generators {
        let Ok(mut transform) = transforms.get_mut(parts.stabalizer) else {
            continue;
        };

        transform.rotate_y(angle);
    }
}

pub fn spawn_generator(commands: &mut Commands, asset_server: &AssetServer) {
    let generator_entity = commands
        .spawn((
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("power_core.glb")),
            ),
            Generator(10),
        ))
        .observe(on_hittable_loaded)
        .id();

    commands.spawn((
        GeneratorHpDisplay(generator_entity),
        Text::new("Generator Health: 10"),
        TextFont::from_font_size(12.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(15),
            width: percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}

use bevy::prelude::*;

use crate::{base::AttackCooldown, damage::Health, enemy::Enemy, shoot::spawn_bullet};

#[derive(Component)]
pub struct Turret;

#[derive(Component)]
pub struct TurretParts {
    pub head: Entity,
    pub muzzle: Entity,
}

#[derive(Resource)]
pub struct TurretSounds {
    pub fire: Handle<AudioSource>,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_turret_sounds);
    app.add_systems(
        Update,
        (resolve_turret_parts, aim_turrets, handle_turret_attack).chain(),
    );
}

pub fn load_turret_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TurretSounds {
        fire: asset_server.load("shot.flac"),
    });
}

pub fn aim_turrets(
    time: Res<Time>,
    enemies: Query<(&GlobalTransform, &Health), With<Enemy>>,
    turrets: Query<&TurretParts, With<Turret>>,
    globals: Query<&GlobalTransform>,
    mut transforms: Query<&mut Transform>,
) {
    let Some((target, _)) = enemies.iter().min_by(|(_, a), (_, b)| a.0.total_cmp(&b.0)) else {
        return;
    };

    let max_angle = 90.0_f32.to_radians() * time.delta_secs();

    for parts in &turrets {
        let Ok(head_global) = globals.get(parts.head) else {
            continue;
        };

        let mut dir = target.translation() - head_global.translation();

        dir.y = 0.0;

        if dir.length_squared() == 0.0 {
            continue;
        }

        let Ok(mut head) = transforms.get_mut(parts.head) else {
            continue;
        };

        let target_rotation = Transform::IDENTITY.looking_to(dir, Vec3::Y).rotation;
        head.rotation = head.rotation.rotate_towards(target_rotation, max_angle)
    }
}

pub fn resolve_turret_parts(
    mut commands: Commands,
    turrets: Query<Entity, (With<Turret>, Without<TurretParts>)>,
    children: Query<&Children>,
    names: Query<&Name>,
) {
    for turret in turrets {
        let mut head = None;
        let mut muzzle = None;

        for entity in children.iter_descendants(turret) {
            let Ok(name) = names.get(entity) else {
                continue;
            };

            match name.as_str() {
                "Bullet origin" => muzzle = Some(entity),
                "Turret Head" => head = Some(entity),
                _ => {}
            }
        }

        let (Some(head), Some(muzzle)) = (head, muzzle) else {
            continue;
        };

        commands.entity(turret).insert(TurretParts { head, muzzle });
    }
}

pub fn handle_turret_attack(
    time: Res<Time>,
    turrets: Query<(&mut AttackCooldown, &TurretParts), With<Turret>>,
    sounds: Res<TurretSounds>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    globals: Query<&GlobalTransform>,
) {
    for (mut cooldown, parts) in turrets {
        cooldown.0.tick(time.delta());
        if !cooldown.0.is_finished() {
            return;
        }

        cooldown.0.reset();

        let Ok(muzzle) = globals.get(parts.muzzle) else {
            continue;
        };

        let pos = muzzle.translation();
        let dir = muzzle.forward().as_vec3();

        spawn_bullet(
            &mut commands,
            &asset_server,
            dir,
            pos,
            10.0, // speed
            33.0, // damage
            0.15, // size
        );

        commands.spawn((
            AudioPlayer::new(sounds.fire.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub fn spawn_turret(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    size: f32,
) {
    commands.spawn((
        Turret,
        AttackCooldown(Timer::from_seconds(0.1, TimerMode::Repeating)),
        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("turret.glb"))),
        Transform::from_translation(position).with_scale(Vec3::splat(size)),
    ));
}

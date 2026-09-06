use bevy::prelude::*;

use crate::game::projectiles::projectile_launcher::ProjectileLauncherPlugin;

pub mod projectile_launcher;

pub const PROJECTILE_SPEED: f32 = 200.0;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProjectileLauncherPlugin);

        app.add_systems(Update, update_projectiles);
    }
}

#[derive(SceneComponent, Clone, FromTemplate)]
#[scene(ProjectileProps)]
pub struct Projectile {
    pub target: Entity,
}

#[derive(Default)]
pub struct ProjectileProps {
    pub spawn_position: Vec2,
}

impl Projectile {
    fn scene(props: ProjectileProps) -> impl Scene {
        bsn! {
            Transform::from_xyz(props.spawn_position.x, props.spawn_position.y, 0.0)

            Sprite {
                custom_size: {Some(Vec2::splat(6.0))},
                color: Color::WHITE,
            }
        }
    }
}

fn update_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(Entity, &mut Transform, &Projectile)>,
    entities: Query<&Transform, Without<Projectile>>,
) {
    for (entity, mut transform, projectile) in &mut projectiles {
        let target = projectile.target;

        match entities.get(target) {
            Ok(target_transform) => {
                let to_target = target_transform.translation - transform.translation;
                let to_target_norm = to_target.normalize();

                if to_target.length() < 2.0 {
                    commands.entity(target).despawn();
                    commands.entity(entity).despawn();
                } else {
                    transform.translation += to_target_norm * PROJECTILE_SPEED * time.delta_secs();
                }
            }
            Err(_) => {
                commands.entity(entity).despawn();
            }
        }
    }
}

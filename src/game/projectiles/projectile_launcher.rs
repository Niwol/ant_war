use bevy::prelude::*;

use crate::{
    AppState,
    game::{ant::Ant, game_info::GameState, player::PlayerRef, projectiles::Projectile},
    world_grid::grid_transform::GridTransform,
};

pub struct ProjectileLauncherPlugin;
impl Plugin for ProjectileLauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_launcher_ranges.run_if(in_state(AppState::InGame)),
        );

        app.add_systems(
            Update,
            upadte_launcher_timers.run_if(in_state(GameState::Playing { paused: false })),
        );
        app.add_systems(
            Update,
            shoot.run_if(in_state(GameState::Playing { paused: false })),
        );
    }
}

#[derive(Component, Default, Clone)]
pub struct ProjectileLauncher {
    range: f32,
    shoot_timer: Timer,
}

impl ProjectileLauncher {
    pub fn new(range: f32, reload_time: f32) -> Self {
        Self {
            range,
            shoot_timer: Timer::from_seconds(reload_time, TimerMode::Once),
        }
    }

    fn can_shoot(&self) -> bool {
        self.shoot_timer.is_finished()
    }
}

fn upadte_launcher_timers(time: Res<Time>, launchers: Query<&mut ProjectileLauncher>) {
    for mut launcher in launchers {
        launcher.shoot_timer.tick(time.delta());
    }
}

fn shoot(
    mut commands: Commands,
    mut launchers: Query<(&mut ProjectileLauncher, &GridTransform, Option<&PlayerRef>)>,
    ants: Query<(Entity, &Transform, &PlayerRef), With<Ant>>,
) {
    for (mut launcher, grid_transform, tower_player_ref) in &mut launchers {
        if !launcher.can_shoot() {
            continue;
        }

        let mut closest_ant = None;
        let launcher_position = grid_transform.center_in_world();

        for (ant_entity, transform, ant_player_ref) in &ants {
            if let Some(player_ref) = tower_player_ref
                && player_ref == ant_player_ref
            {
                continue;
            }

            let dist = (launcher_position - transform.translation.xy()).length();
            match closest_ant {
                Some((entity, closest_dist)) => {
                    if dist < closest_dist {
                        closest_ant = Some((entity, dist));
                    }
                }
                None => {
                    if dist < launcher.range {
                        closest_ant = Some((ant_entity, dist));
                    }
                }
            }
        }

        if let Some((ant_entity, _)) = closest_ant {
            commands.spawn_scene(bsn! {
                @Projectile {
                    target: ant_entity,
                    @spawn_position: launcher_position
                }
            });

            launcher.shoot_timer.reset();
        }
    }
}

fn draw_launcher_ranges(
    mut gizmos: Gizmos,
    launchers: Query<(&ProjectileLauncher, &GridTransform)>,
) {
    for (launcher, grid_transform) in &launchers {
        let center = grid_transform.center_in_world();

        gizmos.circle_2d(center, launcher.range, Color::WHITE);
    }
}

use bevy::prelude::*;

use crate::game::{
    InGameEntity,
    ant::{AntProps, AntTarget, ant_type::AntType, soldier::Soldier, unit::Unit, worker::Worker},
    game_info::GameState,
    player::{Player, PlayerRef},
};

pub struct AntSpawnerPlugin;
impl Plugin for AntSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_ant_spawners.run_if(in_state(GameState::Playing { paused: false })),
        );
    }
}

#[derive(SceneComponent, FromTemplate, Clone)]
#[scene(AntSpawnerProps)]
#[require(InGameEntity)]
pub struct AntSpawner {
    pub ant_type: AntType,
    pub left_to_spawn: u32,
    pub target_building: Entity,
    pub spawn_timer: Timer,
}

impl Default for AntSpawner {
    fn default() -> Self {
        Self {
            ant_type: AntType::Unit,
            left_to_spawn: 0,
            target_building: Entity::PLACEHOLDER,
            spawn_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

#[derive(Default)]
pub struct AntSpawnerProps {
    pub position: Vec2,
}

impl AntSpawner {
    fn scene(props: AntSpawnerProps) -> impl Scene {
        bsn! {
            Transform::from_xyz(props.position.x, props.position.y, 0.0)
        }
    }
}

fn update_ant_spawners(
    time: Res<Time>,
    mut commands: Commands,
    mut ant_spawners: Query<(Entity, &mut AntSpawner, &PlayerRef, &Transform)>,
    players: Query<&Player>,
) {
    for (entity, mut ant_spawner, player_ref, transform) in &mut ant_spawners {
        ant_spawner.spawn_timer.tick(time.delta());

        if ant_spawner.spawn_timer.just_finished() {
            ant_spawner.left_to_spawn -= 1;
            let player = players.get(player_ref.0).unwrap();

            let ant_props = AntProps {
                position: transform.translation.xy(),
            };

            let mut ant_commands = match ant_spawner.ant_type {
                AntType::Unit => commands.spawn_scene(bsn! {
                        @Unit {
                        @ant_props,
                        @player_color: {player.player_color},
                    }
                }),

                AntType::Worker => commands.spawn_scene(bsn! {
                        @Worker {
                        @ant_props,
                        @player_color: {player.player_color},
                    }
                }),

                AntType::Soldier => commands.spawn_scene(bsn! {
                        @Soldier {
                        @ant_props,
                        @player_color: {player.player_color},
                    }
                }),
            };

            ant_commands.insert((
                AntTarget(ant_spawner.target_building),
                PlayerRef(player_ref.0),
            ));
        }

        if ant_spawner.left_to_spawn == 0 {
            commands.entity(entity).despawn();
        }
    }
}

use bevy::prelude::*;

use crate::game::{InGameEntity, ant::SpawnAnt, game_info::GameState, team::PlayerRef};

pub struct AntSpawnerPlugin;
impl Plugin for AntSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_ant_spawners.run_if(in_state(GameState::Playing { paused: false })),
        );

        app.add_observer(spawn_ant_spawner);
    }
}

#[derive(Event)]
pub struct SapwnAntSpawner {
    pub position: Vec2,
    pub nb_to_spawn: u32,
    pub target_building: Entity,
    pub player_ref: PlayerRef,
}

#[derive(Component, Clone)]
#[require(InGameEntity)]
pub struct AntSpawner {
    left_to_spawn: u32,
    spawn_timer: Timer,
    target_building: Entity,
}

impl Default for AntSpawner {
    fn default() -> Self {
        Self {
            left_to_spawn: 0,
            spawn_timer: Timer::default(),
            target_building: Entity::PLACEHOLDER,
        }
    }
}

fn spawn_ant_spawner(spawn: On<SapwnAntSpawner>, mut commands: Commands) {
    let position = spawn.position;

    commands.spawn_scene(bsn! {
        AntSpawner {
            left_to_spawn: { spawn.nb_to_spawn },
            spawn_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            target_building: { spawn.target_building }
        }

        PlayerRef({spawn.player_ref.0})

        Transform::from_xyz(position.x, position.y, 0.0)
    });
}

fn update_ant_spawners(
    time: Res<Time>,
    mut commands: Commands,
    mut ant_spawners: Query<(Entity, &mut AntSpawner, &PlayerRef, &Transform)>,
) {
    for (entity, mut ant_spawner, player_ref, transform) in &mut ant_spawners {
        ant_spawner.spawn_timer.tick(time.delta());

        if ant_spawner.spawn_timer.just_finished() {
            ant_spawner.left_to_spawn -= 1;

            commands.trigger(SpawnAnt {
                player_ref: *player_ref,
                position: transform.translation.xy(),
                target: ant_spawner.target_building,
            });
        }

        if ant_spawner.left_to_spawn == 0 {
            commands.entity(entity).despawn();
        }
    }
}

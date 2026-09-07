use bevy::prelude::*;

use crate::game::{
    InGameEntity,
    ant::{ant_spawner::AntSpawnerPlugin, ant_stats::AntStats, ant_type::AntType},
    building::{Building, EnterBuilding},
    game_info::GameState,
};

pub mod ant_spawner;
pub mod ant_stats;
pub mod ant_type;
pub mod asset_paths;
pub mod soldier;
pub mod unit;
pub mod worker;

pub struct AntPlugin;
impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AntSpawnerPlugin);

        app.add_systems(
            FixedUpdate,
            update_ants.run_if(in_state(GameState::Playing { paused: false })),
        );
        app.add_systems(Update, despawn_dead_ants);
    }
}

#[derive(Component, Clone, FromTemplate)]
pub struct AntTarget(pub Entity);

#[derive(SceneComponent, FromTemplate, Clone)]
#[scene(AntProps)]
#[require(InGameEntity)]
pub struct Ant {
    _ant_type: AntType,
}

#[derive(Default)]
pub struct AntProps {
    pub position: Vec2,
}

impl Ant {
    fn scene(props: AntProps) -> impl Scene {
        bsn! {
            Transform::from_xyz(props.position.x, props.position.y, 0.0)
        }
    }
}

fn update_ants(
    time: Res<Time>,
    mut commands: Commands,
    mut ants: Query<(Entity, &AntTarget, &mut Transform, &AntStats), With<Ant>>,
    buildings: Query<&Transform, (With<Building>, Without<Ant>)>,
) {
    for (entity, ant_target, mut ant_transform, ant_stats) in &mut ants {
        let building = buildings.get(ant_target.0).unwrap();

        let to_building = building.translation - ant_transform.translation;
        let to_building_norm = to_building.normalize_or_zero();

        ant_transform.translation += to_building_norm * ant_stats.speed * time.delta_secs();

        if to_building.length() < 2.0 {
            commands.trigger(EnterBuilding {
                ant: entity,
                building: ant_target.0,
            });
        }
    }
}

fn despawn_dead_ants(mut commands: Commands, ants: Query<(Entity, &AntStats), Changed<AntStats>>) {
    for (ant_entity, ant_stats) in &ants {
        if ant_stats.health.is_dead() {
            commands.entity(ant_entity).despawn();
        }
    }
}

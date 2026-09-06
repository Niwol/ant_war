use bevy::prelude::*;

use crate::game::{
    InGameEntity,
    ant::{ant_spawner::AntSpawnerPlugin, ant_type::AntType},
    building::{Building, inhabitants::Inhabitants},
    game_info::GameState,
    player::PlayerRef,
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
    }
}

#[derive(EntityEvent)]
struct EnterBuilding {
    #[event_target]
    ant: Entity,
    building: Entity,
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
            on(enter_building)
        }
    }
}

fn update_ants(
    time: Res<Time>,
    mut commands: Commands,
    mut ants: Query<(Entity, &AntTarget, &mut Transform), With<Ant>>,
    buildings: Query<&Transform, (With<Building>, Without<Ant>)>,
) {
    for (entity, ant_target, mut ant_transform) in &mut ants {
        let building = buildings.get(ant_target.0).unwrap();

        let to_building = building.translation - ant_transform.translation;
        let to_building_norm = to_building.normalize_or_zero();

        ant_transform.translation += to_building_norm * 50.0 * time.delta_secs();

        if to_building.length() < 2.0 {
            commands.trigger(EnterBuilding {
                ant: entity,
                building: ant_target.0,
            });
        }
    }
}

fn enter_building(
    enter_event: On<EnterBuilding>,
    mut commands: Commands,
    ants: Query<&PlayerRef, (With<Ant>, Without<Building>)>,
    mut buildings: Query<(Option<&mut PlayerRef>, &mut Inhabitants), With<Building>>,
) {
    let (building_player_ref, mut inhabitants) = buildings.get_mut(enter_event.building).unwrap();
    let ant_player_ref = ants.get(enter_event.ant).unwrap();

    if let Some(mut building_player_ref) = building_player_ref {
        if *building_player_ref == *ant_player_ref {
            inhabitants.add(1);
        } else {
            if inhabitants.current() == 0 {
                *building_player_ref = *ant_player_ref;
                inhabitants.add(1);
            } else {
                inhabitants.take(1);
            }
        }
    } else {
        if inhabitants.current() == 0 {
            commands
                .entity(enter_event.building)
                .insert(*ant_player_ref);
            inhabitants.add(1);
        } else {
            inhabitants.take(1);
        }
    }

    commands.entity(enter_event.ant).despawn();
}

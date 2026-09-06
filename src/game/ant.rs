use bevy::prelude::*;

use crate::game::{
    InGameEntity,
    ant::{ant_spawner::AntSpawnerPlugin, ant_type::AntType},
    building::{Building, inhabitants::Inhabitants},
    game_info::GameState,
    player::{PlayerColor, PlayerRef},
};

pub mod ant_spawner;
pub mod ant_type;
pub mod asset_paths;

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

#[derive(SceneComponent, FromTemplate, Clone)]
#[scene(AntProps)]
#[require(InGameEntity)]
pub struct Ant {
    _ant_type: AntType,
    target_building: Entity,
}

#[derive(Default)]
pub struct AntProps {
    ant_type: AntType,
    position: Vec2,
    player_color: PlayerColor,
}

impl Ant {
    fn scene(props: AntProps) -> impl Scene {
        let image_path = asset_paths::get_path(props.ant_type, props.player_color);

        bsn! {
            Sprite {
                image: image_path,
            }

            Transform::from_xyz(props.position.x, props.position.y, 0.0)
            on(enter_building)
        }
    }
}

fn update_ants(
    time: Res<Time>,
    mut commands: Commands,
    mut ants: Query<(Entity, &mut Transform, &Ant)>,
    buildings: Query<&Transform, (With<Building>, Without<Ant>)>,
) {
    for (entity, mut ant_transform, ant) in &mut ants {
        let building = buildings.get(ant.target_building).unwrap();

        let to_building = building.translation - ant_transform.translation;
        let to_building_norm = to_building.normalize_or_zero();

        ant_transform.translation += to_building_norm * 50.0 * time.delta_secs();

        if to_building.length() < 2.0 {
            commands.trigger(EnterBuilding {
                ant: entity,
                building: ant.target_building,
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

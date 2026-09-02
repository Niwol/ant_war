use bevy::{ecs::query::QueryData, platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    game::{
        InGameEntity,
        ant::ant_spawner::SapwnAntSpawner,
        bot::Bot,
        building::{
            asset_paths::*,
            building_selection::{BuildingSelectionPlugin, SelectedBuildings},
            building_type::BuildingType,
            house::House,
            inhabitants::Inhabitants,
            main_building::MainBuilding,
            tower::Tower,
        },
        input::{self, InputMoveOrder},
        player::{Player, PlayerColor, PlayerRef},
    },
    world_grid::grid_transform::GridTransform,
};

pub mod asset_paths;
pub mod building_selection;
pub mod building_type;
pub mod house;
pub mod inhabitants;
mod main_building;
pub mod tower;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            house::plugin,
            main_building::plugin,
            tower::plugin,
            inhabitants::plugin,
            BuildingSelectionPlugin,
        ));

        app.add_systems(Startup, load_building_sprites);

        app.add_systems(Update, player_changed);

        app.add_observer(spawn_building);
        app.add_observer(on_input_move_order);
    }
}

#[derive(Resource)]
pub struct BuildingSprites {
    pub house_sprites: HashMap<PlayerColor, Handle<Image>>,
    pub main_building_sprites: HashMap<PlayerColor, Handle<Image>>,
    pub tower_sprites: HashMap<PlayerColor, Handle<Image>>,
}

impl BuildingSprites {
    pub fn get(&self, building_type: BuildingType, player_color: PlayerColor) -> Handle<Image> {
        match building_type {
            BuildingType::House => self.house_sprites[&player_color].clone(),
            BuildingType::MainBuilding { index: _ } => {
                self.main_building_sprites[&player_color].clone()
            }
            BuildingType::Tower => self.tower_sprites[&player_color].clone(),
        }
    }
}

fn load_building_sprites(mut commands: Commands, assets: Res<AssetServer>) {
    let building_sprites = BuildingSprites {
        house_sprites: HashMap::from([
            (PlayerColor::Blue, assets.load(PATH_HOUSE_BLUE)),
            (PlayerColor::Red, assets.load(PATH_HOUSE_RED)),
            (PlayerColor::Green, assets.load(PATH_HOUSE_GREEN)),
            (PlayerColor::Orange, assets.load(PATH_HOUSE_ORANGE)),
            (PlayerColor::Purple, assets.load(PATH_HOUSE_PURPLE)),
            (PlayerColor::Yellow, assets.load(PATH_HOUSE_YELLOW)),
            (PlayerColor::Aqua, assets.load(PATH_HOUSE_AQUA)),
            (PlayerColor::Pink, assets.load(PATH_HOUSE_PINK)),
            (PlayerColor::Neutral, assets.load(PATH_HOUSE_NEUTRAL)),
        ]),

        main_building_sprites: HashMap::from([
            (PlayerColor::Blue, assets.load(PATH_MAIN_BUILDING_BLUE)),
            (PlayerColor::Red, assets.load(PATH_MAIN_BUILDING_RED)),
            (PlayerColor::Green, assets.load(PATH_MAIN_BUILDING_GREEN)),
            (PlayerColor::Orange, assets.load(PATH_MAIN_BUILDING_ORANGE)),
            (PlayerColor::Purple, assets.load(PATH_MAIN_BUILDING_PURPLE)),
            (PlayerColor::Yellow, assets.load(PATH_MAIN_BUILDING_YELLOW)),
            (PlayerColor::Aqua, assets.load(PATH_MAIN_BUILDING_AQUA)),
            (PlayerColor::Pink, assets.load(PATH_MAIN_BUILDING_PINK)),
            (
                PlayerColor::Neutral,
                assets.load(PATH_MAIN_BUILDING_NEUTRAL),
            ),
        ]),

        tower_sprites: HashMap::from([
            (PlayerColor::Blue, assets.load(PATH_TOWER_BLUE)),
            (PlayerColor::Red, assets.load(PATH_TOWER_RED)),
            (PlayerColor::Green, assets.load(PATH_TOWER_GREEN)),
            (PlayerColor::Orange, assets.load(PATH_TOWER_ORANGE)),
            (PlayerColor::Purple, assets.load(PATH_TOWER_PURPLE)),
            (PlayerColor::Yellow, assets.load(PATH_TOWER_YELLOW)),
            (PlayerColor::Aqua, assets.load(PATH_TOWER_AQUA)),
            (PlayerColor::Pink, assets.load(PATH_TOWER_PINK)),
            (PlayerColor::Neutral, assets.load(PATH_TOWER_NEUTRAL)),
        ]),
    };

    commands.insert_resource(building_sprites);
}

#[derive(EntityEvent)]
pub struct MoveOrder {
    pub entity: Entity,
    pub target: Entity,
}

#[derive(SceneComponent, Default, Clone)]
#[require(Pickable, Sprite, InGameEntity)]
#[scene(BuildingProps)]
pub struct Building {
    building_type: BuildingType,
}

#[derive(Default, Clone, Copy)]
pub struct BuildingProps {
    building_id: BuildingId,
    grid_transform: GridTransform,
}

impl Building {
    fn scene(props: BuildingProps) -> impl Scene {
        bsn! {
            template_value(props.building_id)
            template_value(props.grid_transform)

            on(input::on_select)
            on(input::on_order)
            on(on_move_order)
        }
    }
}

#[derive(Component, Serialize, Deserialize, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BuildingId(usize);

impl BuildingId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(QueryData)]
pub struct BuildingTypeQueryData {
    pub house: Option<&'static House>,
    pub main_building: Option<&'static MainBuilding>,
}

#[derive(Event)]
pub struct SpawnBuilding {
    pub building_id: BuildingId,
    pub building_type: BuildingType,
    pub grid_transform: GridTransform,
}

fn spawn_building(spawn: On<SpawnBuilding>, mut commands: Commands) {
    let building_props = BuildingProps {
        building_id: spawn.building_id,
        grid_transform: spawn.grid_transform,
    };

    match spawn.building_type {
        BuildingType::House => {
            commands.spawn_scene(bsn! {
                @House {
                    @building_props
                }
            });
        }
        BuildingType::MainBuilding { index } => {
            commands.spawn_scene(bsn! {
                @MainBuilding {
                    @building_props,
                    @main_building_index: index,
                }
            });
        }
        BuildingType::Tower => {
            commands.spawn_scene(bsn! {
                @Tower {
                    @building_props
                }
            });
        }
    }
}

fn on_move_order(
    order: On<MoveOrder>,
    mut commands: Commands,
    mut buildings: Query<(&mut Inhabitants, &PlayerRef, &Transform)>,
) {
    let (mut inhabitants, player_ref, transform) = buildings.get_mut(order.entity).unwrap();

    let to_move = inhabitants.current() / 2 + inhabitants.current() % 2;
    inhabitants.take(to_move);

    commands.trigger(SapwnAntSpawner {
        position: transform.translation.xy(),
        nb_to_spawn: to_move as u32,
        target_building: order.target,
        player_ref: player_ref.clone(),
    });
}

fn player_changed(
    players: Query<&Player>,
    mut sprites: Query<(&mut Sprite, Option<&PlayerRef>, &Building)>,

    building_sprites: Res<BuildingSprites>,
) {
    for (mut sprite, player_ref, building) in &mut sprites {
        let building_type = building.building_type;

        let player_color = if let Some(player_ref) = player_ref {
            let player = players.get(player_ref.0).unwrap();
            player.player_color
        } else {
            PlayerColor::Neutral
        };
        let image_handle = building_sprites.get(building_type, player_color);

        sprite.image = image_handle;
    }
}

fn on_input_move_order(
    order: On<InputMoveOrder>,
    mut commands: Commands,
    selected_buildings: Res<SelectedBuildings>,
    buildings: Query<&PlayerRef, With<Building>>,
    players: Query<Entity, (With<Player>, Without<Bot>)>,
) {
    for selected_building in &selected_buildings.buildings {
        if let Ok(player_ref) = buildings.get(*selected_building) {
            if players.contains(player_ref.0) {
                commands.trigger(MoveOrder {
                    entity: *selected_building,
                    target: order.target,
                });
            }
        }
    }
}

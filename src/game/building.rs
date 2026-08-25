use bevy::{ecs::query::QueryData, platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    game::{
        InGameEntity,
        ant::ant_spawner::SapwnAntSpawner,
        building::{
            asset_paths::*, house::HouseMarker, inhabitants::Inhabitants,
            main_building::MainBuilding,
        },
        game_info::GameState,
        input::{self, MoveOrder},
        team::{Player, PlayerColor, PlayerRef},
    },
    world_grid::grid_transform::GridTransform,
};

pub mod asset_paths;
pub mod house;
pub mod inhabitants;
mod main_building;

pub struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((house::plugin, main_building::plugin, inhabitants::plugin));

        app.add_systems(Startup, load_building_sprites);

        app.add_systems(
            Update,
            building_grow.run_if(in_state(GameState::Playing { paused: false })),
        );
        app.add_systems(Update, player_changed);

        app.add_observer(spawn_building);
    }
}

#[derive(Resource)]
pub struct BuildingSprites {
    pub house_sprites: HashMap<PlayerColor, Handle<Image>>,
    pub main_building_sprites: HashMap<PlayerColor, Handle<Image>>,
}

impl BuildingSprites {
    pub fn get(&self, building_type: BuildingType, player_color: PlayerColor) -> Handle<Image> {
        match building_type {
            BuildingType::House => self.house_sprites[&player_color].clone(),
            BuildingType::MainBuilding { index: _ } => {
                self.main_building_sprites[&player_color].clone()
            }
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
    };

    commands.insert_resource(building_sprites);
}

#[derive(Component, Default, Clone)]
#[require(Pickable, Sprite, InGameEntity)]
pub struct Building {
    max_inhabitants: i32,
    growable: bool,
    grow_timer: Timer,
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

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildingType {
    #[default]
    House,
    MainBuilding {
        index: usize,
    },
}

impl BuildingType {
    pub fn grid_size(&self) -> UVec2 {
        match self {
            BuildingType::House => UVec2::splat(3),
            BuildingType::MainBuilding { index: _ } => UVec2::splat(4),
        }
    }
}

#[derive(QueryData)]
pub struct BuildingTypeQueryData {
    house: Option<&'static HouseMarker>,
    main_building: Option<&'static MainBuilding>,
}

#[derive(Event)]
pub struct SpawnBuilding {
    pub building_type: BuildingType,
    pub player_ref: Option<PlayerRef>,
    pub grid_transform: GridTransform,
    pub inhabitants: i32,
}

fn spawn_building(spawn: On<SpawnBuilding>, mut commands: Commands) {
    let inhabitants = spawn.inhabitants;

    let mut building = commands.spawn_scene(bsn! {
        Building

        Inhabitants::new(inhabitants)

        template_value(spawn.grid_transform)
    });

    if let Some(player_ref) = &spawn.player_ref {
        building.insert(player_ref.clone());
    }

    match spawn.building_type {
        BuildingType::House => building.insert(HouseMarker),
        BuildingType::MainBuilding { index: _ } => building.insert(MainBuilding),
    };

    building
        .observe(input::on_select)
        .observe(input::on_order.run_if(in_state(GameState::Playing { paused: false })))
        .observe(on_move_order);
}

fn on_move_order(
    order: On<MoveOrder>,
    mut commands: Commands,
    mut buildings: Query<(&mut Inhabitants, &PlayerRef, &Transform)>,
) {
    let (mut inhabitants, player_ref, transform) = buildings.get_mut(order.entity).unwrap();

    let to_move = inhabitants.total() / 2 + inhabitants.total() % 2;
    inhabitants.take(to_move);

    commands.trigger(SapwnAntSpawner {
        position: transform.translation.xy(),
        nb_to_spawn: to_move as u32,
        target_building: order.target,
        player_ref: player_ref.clone(),
    });
}

fn building_grow(
    time: Res<Time>,
    mut player_buildings: Query<(&mut Building, &mut Inhabitants), With<PlayerRef>>,
) {
    for (mut building, mut inhabitants) in &mut player_buildings {
        building.grow_timer.tick(time.delta());

        if building.grow_timer.just_finished() {
            if building.max_inhabitants > inhabitants.total() && building.growable {
                inhabitants.add(1);
            } else if building.max_inhabitants < inhabitants.total() {
                inhabitants.take(1);
            }
        }
    }
}

fn player_changed(
    players: Query<&Player>,
    mut sprites: Query<
        (&mut Sprite, Option<&PlayerRef>, BuildingTypeQueryData),
        (With<Building>, Changed<PlayerRef>),
    >,

    building_sprites: Res<BuildingSprites>,
) {
    for (mut sprite, player_ref, building_type) in &mut sprites {
        let building_type = if building_type.house.is_some() {
            BuildingType::House
        } else if building_type.main_building.is_some() {
            BuildingType::MainBuilding { index: 0 }
        } else {
            unreachable!("No building type");
        };

        let player_color = if let Some(player_ref) = player_ref {
            let player = players.get(player_ref.0).unwrap();
            player.player_color()
        } else {
            PlayerColor::Neutral
        };
        let image_handle = building_sprites.get(building_type, player_color);

        sprite.image = image_handle;
    }
}

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    AppState, MainCamera,
    game::{
        ant::AntPlugin,
        bot::{Bot, BotPlugin, brain::Brain},
        building::{BuildingPlugin, BuildingType, SpawnBuilding},
        game_info::{GameInfoPlugin, StartGameInfo},
        input::InputPlugin,
        team::{Player, PlayerRef, Team},
    },
    map::Map,
    menu::game_preparation_menu::menu_backend::GamePreparationInfo,
    world_grid::WorldGrid,
};

pub mod ant;
pub mod bot;
pub mod building;
pub mod game_info;
pub mod input;
pub mod team;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameInfoPlugin,
            AntPlugin,
            BotPlugin,
            BuildingPlugin,
            InputPlugin,
        ));

        app.add_systems(OnExit(AppState::InGame), cleanup_in_game_entities);

        app.add_observer(start_game);
        app.add_observer(spawn_map);
    }
}

#[derive(Component, Default, Clone)]
struct InGameEntity;

fn cleanup_in_game_entities(
    mut commands: Commands,
    in_game_entities: Query<Entity, With<InGameEntity>>,
) {
    for entity in &in_game_entities {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<WorldGrid>();
}

#[derive(Event)]
pub struct StartGame {
    pub map: Map,
    pub game_preparation_info: GamePreparationInfo,
}

#[derive(Event)]
pub struct SpawnMap {
    pub map: Map,
    pub game_preparation_info: GamePreparationInfo,
}

fn start_game(
    start_game: On<StartGame>,
    mut commands: Commands,
    mut window: Single<&mut Window>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let map_size = start_game.map.size_world();

    window.resolution.set(map_size.x + 50.0, map_size.y + 50.0);

    commands.trigger(SpawnMap {
        map: start_game.map.clone(),
        game_preparation_info: start_game.game_preparation_info.clone(),
    });
    next_state.set(AppState::InGame);
}

fn spawn_map(
    spawn_map: On<SpawnMap>,
    mut commands: Commands,
    camera_transform: Single<&mut Transform, With<MainCamera>>,
) {
    let map = spawn_map.map.clone();
    let mut start_game_info = StartGameInfo {
        players: Vec::new(),
        building_assignements: HashMap::new(),
    };

    let world_grid = WorldGrid::new(map.size());
    let map_center = world_grid.center();

    *camera_transform.into_inner() = Transform::from_xyz(map_center.x, map_center.y, 0.0);
    commands.insert_resource(world_grid);

    let mut player_refs = HashMap::new();

    for player_info in spawn_map.game_preparation_info.player_infos() {
        let mut player_commands = commands.spawn_scene(bsn! {
            Player::new(
                Team::new(player_info.player_id as u32),
                player_info.player_color,
            )
        });

        if player_info.bot {
            player_commands.insert(Bot::new(Brain::simple()));
        }

        let player_entity = player_commands.id();

        start_game_info.players.push(player_entity);

        player_refs.insert(player_info.player_id, PlayerRef(player_entity));
    }

    for (building_id, building_info) in map.building_infos() {
        let building_info = building_info;

        let mut player_ref = None;
        match building_info.building_type {
            BuildingType::House => (),
            BuildingType::MainBuilding { index } => {
                let player_id = spawn_map.game_preparation_info.building_assignements[&index];
                player_ref = player_refs.get(&player_id).copied();
            }
        }

        if let Some(player_ref) = player_ref {
            start_game_info
                .building_assignements
                .insert(building_id, player_ref);
        }

        commands.trigger(SpawnBuilding {
            building_id,
            building_type: building_info.building_type,
            grid_transform: building_info.grid_transform,
        });
    }

    commands.insert_resource(start_game_info);
}

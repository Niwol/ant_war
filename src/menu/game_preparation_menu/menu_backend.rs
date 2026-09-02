use bevy::{
    platform::collections::HashMap, prelude::*, ui::InteractionDisabled, ui_widgets::Activate,
};

use crate::{
    game::{
        StartGame,
        building::{self, BuildingType},
        player::{PLAYER_COLOR_LIST, PlayerColor},
    },
    map::{Map, MapAccess, MapCollection},
    menu::{
        MenuState,
        game_preparation_menu::{
            BuildingAssignementButton, ButtonTextRef, ColorSelectButton, PlayButton, PlayerId,
            map_preview::MainBuildingIndex,
        },
    },
};

use super::{MapSelected, RespwanMapPrepatationMenu};

pub fn plugin(app: &mut App) {
    app.init_resource::<SelectedMap>();
    app.add_sub_state::<MapLoadingState>();

    app.add_systems(
        Update,
        loading_map.run_if(in_state(MapLoadingState::Loading)),
    );

    app.add_systems(
        Update,
        update_play_button_interactibility.run_if(in_state(MenuState::GamePreparation)),
    );

    app.add_observer(on_map_selected);

    app.add_observer(update_building_assignement_text);
    app.add_observer(update_player_color_button);

    app.add_observer(update_building_assignement_button_border_player_color_change);
    app.add_observer(update_building_assignement_button_border_assignement_change);
    app.add_observer(update_preview_building_color_player_color_change);
    app.add_observer(update_preview_building_color_assignement_change);
}

#[derive(SubStates, PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
#[source(MenuState = MenuState::GamePreparation)]
pub enum MapLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource, Default)]
pub struct SelectedMap {
    _path: String,
    pub handle: Handle<Map>,
}

#[derive(Clone, Copy)]
pub struct PlayerInfo {
    pub player_id: usize,
    pub player_color: PlayerColor,
    pub bot: bool,
}

#[derive(Resource, Clone)]
pub struct GamePreparationInfo {
    pub players: HashMap<usize, PlayerInfo>,
    pub building_assignements: HashMap<usize, usize>,
}

impl GamePreparationInfo {
    fn game_ready(&self) -> bool {
        for (player_id, _player) in &self.players {
            // Each player has at least one building assigned
            if !self
                .building_assignements
                .values()
                .any(|assigned_player| player_id == assigned_player)
            {
                return false;
            }
        }

        true
    }

    pub fn player_infos(&self) -> Vec<PlayerInfo> {
        self.players.values().copied().collect::<Vec<_>>()
    }
}

fn on_map_selected(
    selected: On<MapSelected>,
    mut commands: Commands,
    map_collection: Res<MapCollection>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<MapLoadingState>>,
) {
    let map_info = map_collection.map_info(selected.map_name.clone()).unwrap();

    let handle = match map_info._map_access() {
        MapAccess::Handle(handle) => handle.clone(),
        MapAccess::AssetPath(path) => asset_server.load(path),
    };

    let selected_map = SelectedMap {
        _path: map_info.path(),
        handle,
    };

    commands.insert_resource(selected_map);
    next_state.set(MapLoadingState::Loading);
}

fn loading_map(
    mut commands: Commands,
    selected_map: Res<SelectedMap>,
    maps: Res<Assets<Map>>,
    mut next_state: ResMut<NextState<MapLoadingState>>,
) {
    let map = maps.get(&selected_map.handle);
    if let Some(map) = map {
        let mut game_preparation_info = GamePreparationInfo {
            players: HashMap::new(),
            building_assignements: HashMap::new(),
        };

        game_preparation_info.players = (1..=map.max_players())
            .into_iter()
            .map(|player_id| {
                (
                    player_id,
                    PlayerInfo {
                        player_id,
                        player_color: PlayerColor::from(PLAYER_COLOR_LIST[player_id - 1]),
                        bot: player_id != 1,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        game_preparation_info.building_assignements = map
            .building_infos_as_vec()
            .into_iter()
            .filter_map(|building_info| match building_info.building_type {
                BuildingType::House => None,
                BuildingType::MainBuilding { index } => Some((index, 0)),
                BuildingType::Tower => None,
            })
            .collect::<HashMap<_, _>>();

        commands.trigger(RespwanMapPrepatationMenu {
            map: map.clone(),
            game_preparation_info: game_preparation_info.clone(),
        });

        commands.insert_resource(game_preparation_info);
        next_state.set(MapLoadingState::Loaded);
    }
}

#[derive(Event)]
struct PlayerColorChanged {
    player_id: usize,
    new_color: PlayerColor,
}

#[derive(Event)]
struct BuildingAssignementChanged {
    building_id: usize,
    new_player_id: usize,
}

pub(super) fn on_player_color_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut game_preparation_info: ResMut<GamePreparationInfo>,
    color_select_buttons: Query<&ColorSelectButton>,
) {
    let player_id = color_select_buttons.get(click.entity).unwrap().player_id;

    let present_colors = game_preparation_info
        .players
        .values()
        .filter_map(|player_info| {
            if player_info.player_id == player_id {
                return None;
            }

            Some(player_info.player_color)
        })
        .collect::<Vec<_>>();

    let mut new_color_found = false;
    let mut player_color = game_preparation_info.players[&player_id].player_color;

    while !new_color_found {
        match click.button {
            PointerButton::Primary => {
                player_color = player_color.next_color();
            }
            PointerButton::Secondary => {
                player_color = player_color.previous_color();
            }
            PointerButton::Middle => (),
        }

        if !present_colors.contains(&player_color) {
            new_color_found = true;
        }
    }

    game_preparation_info
        .players
        .get_mut(&player_id)
        .unwrap()
        .player_color = player_color;

    commands.trigger(PlayerColorChanged {
        player_id,
        new_color: player_color,
    });
}

pub(super) fn on_bot_button_clicked(
    click: On<Activate>,
    mut game_preparation_info: ResMut<GamePreparationInfo>,
    player_ids: Query<&PlayerId>,
    button_text_refs: Query<(&ButtonTextRef, &ChildOf)>,
    mut texts: Query<&mut Text>,
) {
    let (button_text_ref, child_of) = button_text_refs.get(click.entity).unwrap();

    let player_id = player_ids.get(child_of.0).unwrap();
    let player_info = game_preparation_info.players.get_mut(&player_id.0).unwrap();
    player_info.bot = !player_info.bot;

    let mut text = texts.get_mut(button_text_ref.entity).unwrap();

    match player_info.bot {
        true => text.0 = "Bot".to_string(),
        false => text.0 = "Player".to_string(),
    }
}

pub(super) fn on_building_assignement_button_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut game_preparation_info: ResMut<GamePreparationInfo>,
    mut building_assignement_buttons: Query<(&BuildingAssignementButton, &mut BorderColor)>,
) {
    let nb_players = game_preparation_info.players.len();

    let assignement_button = building_assignement_buttons.get(click.entity).unwrap().0;

    let mut assigned_player =
        game_preparation_info.building_assignements[&assignement_button.building_id];

    match click.button {
        PointerButton::Primary => {
            assigned_player += 1;
            assigned_player = assigned_player % (nb_players + 1);
        }
        PointerButton::Secondary => {
            if assigned_player == 0 {
                assigned_player = nb_players + 1;
            }
            assigned_player -= 1;
        }

        _ => (),
    }

    *game_preparation_info
        .building_assignements
        .get_mut(&assignement_button.building_id)
        .unwrap() = assigned_player;

    commands.trigger(BuildingAssignementChanged {
        building_id: assignement_button.building_id,
        new_player_id: assigned_player,
    });

    let mut border_color = building_assignement_buttons
        .get_mut(click.entity)
        .unwrap()
        .1;

    if let Some(player_info) = game_preparation_info.players.get(&assigned_player) {
        *border_color = BorderColor::all(player_info.player_color.color());
    } else {
        *border_color = BorderColor::all(PlayerColor::Neutral.color());
    }
}

fn update_building_assignement_text(
    assignement_change: On<BuildingAssignementChanged>,
    building_assignement_buttons: Query<(Entity, &BuildingAssignementButton)>,
    mut texts: Query<(&mut Text, &ChildOf)>,
) {
    let mut button_entity = Entity::PLACEHOLDER;
    for (entity, building_assignement_button) in &building_assignement_buttons {
        if assignement_change.building_id == building_assignement_button.building_id {
            button_entity = entity;
            break;
        }
    }

    for (mut text, child_of) in &mut texts {
        if child_of.0 == button_entity {
            text.0 = format!(
                "{}",
                if assignement_change.new_player_id == 0 {
                    String::from("-")
                } else {
                    format!("{}", assignement_change.new_player_id)
                }
            );
            break;
        }
    }
}

fn update_player_color_button(
    color_change: On<PlayerColorChanged>,
    mut color_select_buttons: Query<(&mut BackgroundColor, &ColorSelectButton)>,
) {
    for (mut background_color, color_select_button) in &mut color_select_buttons {
        if color_change.player_id == color_select_button.player_id {
            background_color.0 = color_change.new_color.color();
            return;
        }
    }
}

fn update_building_assignement_button_border_player_color_change(
    color_change: On<PlayerColorChanged>,
    game_preparation_info: Res<GamePreparationInfo>,
    mut building_assignement_buttons: Query<(&mut BorderColor, &BuildingAssignementButton)>,
) {
    for (mut border_color, building_assignement_button) in &mut building_assignement_buttons {
        let player_id =
            game_preparation_info.building_assignements[&building_assignement_button.building_id];

        if color_change.player_id == player_id {
            *border_color = BorderColor::all(color_change.new_color.color());
        }
    }
}

fn update_building_assignement_button_border_assignement_change(
    assignement_change: On<BuildingAssignementChanged>,
    game_preparation_info: Res<GamePreparationInfo>,
    mut assignement_buttons: Query<(&mut BorderColor, &BuildingAssignementButton)>,
) {
    let player_color = match game_preparation_info
        .players
        .get(&assignement_change.new_player_id)
    {
        Some(player_info) => player_info.player_color,
        None => PlayerColor::Neutral,
    };

    for (mut border_color, assignement_button) in &mut assignement_buttons {
        if assignement_button.building_id == assignement_change.building_id {
            *border_color = BorderColor::all(player_color.color());
        }
    }
}

fn update_preview_building_color_player_color_change(
    color_change: On<PlayerColorChanged>,
    game_preparation_info: Res<GamePreparationInfo>,
    mut sprites: Query<(&mut Sprite, &MainBuildingIndex)>,
    assets: Res<AssetServer>,
) {
    for (mut sprite, main_building_index) in &mut sprites {
        let player_id = game_preparation_info.building_assignements[&main_building_index.0];

        if color_change.player_id == player_id {
            sprite.image = assets.load(building::asset_paths::get_path(
                building::BuildingType::MainBuilding { index: 0 },
                color_change.new_color,
            ));
        }
    }
}

fn update_preview_building_color_assignement_change(
    assignement_change: On<BuildingAssignementChanged>,
    game_preparation_info: Res<GamePreparationInfo>,
    mut sprites: Query<(&mut Sprite, &MainBuildingIndex)>,
    asset_server: Res<AssetServer>,
) {
    let player_color = match game_preparation_info
        .players
        .get(&assignement_change.new_player_id)
    {
        Some(player_info) => player_info.player_color,
        None => PlayerColor::Neutral,
    };

    for (mut sprite, main_building_index) in &mut sprites {
        if main_building_index.0 == assignement_change.building_id {
            sprite.image = asset_server.load(building::asset_paths::get_path(
                building::BuildingType::MainBuilding { index: 0 },
                player_color,
            ));
        }
    }
}

fn update_play_button_interactibility(
    mut commands: Commands,
    game_preparation_info: If<Res<GamePreparationInfo>>,
    play_button: Single<Entity, With<PlayButton>>,
) {
    if !game_preparation_info.is_changed() {
        return;
    }

    if game_preparation_info.game_ready() {
        commands
            .entity(*play_button)
            .remove::<InteractionDisabled>();
    } else {
        commands.entity(*play_button).insert(InteractionDisabled);
    }
}

pub fn play_button_clicked(
    _: On<Activate>,
    mut commands: Commands,
    selected_map: Res<SelectedMap>,
    game_preparation_info: Res<GamePreparationInfo>,
    maps: Res<Assets<Map>>,
) {
    let map = maps.get(&selected_map.handle).unwrap().clone();

    commands.trigger(StartGame {
        map,
        game_preparation_info: game_preparation_info.clone(),
    });
}

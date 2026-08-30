use bevy::{
    color::palettes::css::WHITE,
    feathers::{
        controls::{FeathersButton, FeathersListRow, FeathersListView},
        display::label,
        theme::ThemedText,
    },
    prelude::*,
    ui::{InteractionDisabled, Selected},
    ui_widgets::{Activate, listbox_update_selection},
};

use crate::{
    game::player::{NB_PLAYER_COLORS, PlayerColor},
    map::{Map, MapCollection, MapInfo},
    menu::{
        MenuState,
        game_preparation_menu::{
            map_preview::MapPreviewImage,
            menu_backend::{
                GamePreparationInfo, PlayerInfo, on_bot_button_clicked,
                on_building_assignement_button_clicked, on_player_color_clicked,
                play_button_clicked,
            },
        },
    },
};

pub mod map_preview;
pub mod menu_backend;

const MAP_PREVIEW_IMAGE_SIZE: UVec2 = UVec2 { x: 500, y: 500 };

pub struct GamePreparationMenuPlugin;
impl Plugin for GamePreparationMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((menu_backend::plugin, map_preview::plugin));

        app.add_systems(
            OnEnter(MenuState::GamePreparation),
            spawn_game_preparation_menu,
        );
        app.add_systems(
            OnExit(MenuState::GamePreparation),
            despawn_game_preparation_menu,
        );

        app.add_observer(on_map_loaded);
    }
}

#[derive(Event)]
struct MapSelected {
    path: String,
}

#[derive(Event)]
pub struct RespwanMapPrepatationMenu {
    map: Map,
    game_preparation_info: GamePreparationInfo,
}

#[derive(Component, Default, Clone)]
struct GamePreparationMenu;

fn spawn_game_preparation_menu(mut commands: Commands, maps: Res<MapCollection>) {
    commands.spawn_scene(bsn! {
        GamePreparationMenu
        Node {
            flex_direction: FlexDirection::Column,

            justify_content: JustifyContent::SpaceBetween,

            width: percent(100.0),
            height: percent(100.0),

            border: px(5.0),
            border_radius: px(10.0),
            padding: px(10.0),

            row_gap: px(5.0),
        }
        BorderColor::all(WHITE)

        Children [
            map_selection(&maps),
            middle_panel(),
            bottom_bar(),
        ]
    });
}

fn despawn_game_preparation_menu(
    mut commands: Commands,
    menu: Single<Entity, With<GamePreparationMenu>>,
) {
    commands.entity(*menu).despawn();
}

fn map_selection(maps: &MapCollection) -> impl Scene {
    let mut map_infos = maps.map_infos();
    map_infos.sort_by(|info_1, info_2| info_1.map_name().cmp(&info_2.map_name()));

    let mut map_list = Vec::new();
    for (i, map_info) in map_infos.iter().enumerate() {
        let selected = if i == 0 { true } else { false };
        map_list.push(Box::new(list_row_element(map_info.clone(), selected)));
    }

    bsn! {
        Node {
            max_width: percent(30.0),
            max_height: percent(20.0),
        }

        @FeathersListView {
            @rows: {
                bsn_list! [
                    {map_list}
                ]
            }
        }
        on(listbox_update_selection)
    }
}

fn list_row_element(map_info: MapInfo, selected: bool) -> impl Scene {
    let selected: Box<dyn Scene> = if selected {
        Box::new(bsn! {Selected })
    } else {
        Box::new(bsn! {})
    };

    let name = map_info.map_name();
    let map_path = map_info.path();

    bsn! {
        {selected}
        @FeathersListRow
        Children [Text::new(&name) ThemedText]
        on(move |_add: On<Add, Selected>, mut commands: Commands| {
            commands.trigger(MapSelected {
                path: map_path.clone(),
            });
        })
    }
}

#[derive(Component, Default, Clone)]
struct MiddlePanelMarker;

fn middle_panel() -> impl Scene {
    bsn! {
        MiddlePanelMarker
        Node {
            justify_self: JustifySelf::Stretch,

            width: percent(100.0),
            height: percent(100.0),

            border: px(2.0),
            border_radius: px(2.0),
            padding: px(5.0),
        }
        BorderColor::all(WHITE)
    }
}

fn on_map_loaded(
    respawn: On<RespwanMapPrepatationMenu>,
    mut commands: Commands,
    node: Single<Entity, With<MiddlePanelMarker>>,
    map_preview_image: Res<MapPreviewImage>,
) {
    let map = respawn.map.clone();
    let game_preparation_info = respawn.game_preparation_info.clone();

    commands.entity(*node).despawn_children();

    let node_content = commands
        .spawn_scene(middle_panel_content(
            map,
            game_preparation_info,
            map_preview_image.handle.clone(),
        ))
        .id();

    commands.entity(*node).add_child(node_content);
}

fn middle_panel_content(
    _map: Map,
    game_preparation_info: GamePreparationInfo,
    image: Handle<Image>,
) -> impl Scene {
    bsn! {
        Node {
            align_content: AlignContent::Center,
            justify_content: JustifyContent::SpaceAround,

            width: percent(100.0),
            height: percent(100.0),
        }

        Children [
            left_panel(game_preparation_info),
            map_preview_image(image),
        ]
    }
}

fn left_panel(game_preparation_info: GamePreparationInfo) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,

            align_self: AlignSelf::Center,
            align_content: AlignContent::Center,

            border: px(2.0),
            border_radius: px(2.0),
            padding: px(5.0),

            row_gap: px(5.0),
        }
        BorderColor::all(WHITE)

        Children [
            player_informations(game_preparation_info.clone()),
            building_assignements(game_preparation_info),
        ]
    }
}

fn player_informations(game_preparation_info: GamePreparationInfo) -> impl Scene {
    let mut player_list = Vec::new();

    let mut players = game_preparation_info.players.values().collect::<Vec<_>>();

    players.sort_by(|p1, p2| p1.player_id.cmp(&p2.player_id));

    for player_info in players {
        let player_element = player_element(player_info.player_id, *player_info);
        player_list.push(player_element);
    }

    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Center,

            row_gap: px(3.0),

            border: px(2.0),
            border_radius: px(2.0),
            padding: px(5.0),
        }
        BorderColor::all(WHITE)

        Children [
            {player_list}
        ]
    }
}

#[derive(SceneComponent, Default, Clone)]
#[scene(ColorSelectButtonProps)]
struct ColorSelectButton {
    player_id: usize,
}

#[derive(Default)]
struct ColorSelectButtonProps {
    player_color: PlayerColor,
}

impl ColorSelectButton {
    fn scene(props: ColorSelectButtonProps) -> impl Scene {
        bsn! {
            Node {
                border_radius: px(3.0),
                width: px(16.0),
                height: px(16.0),
            }
            ColorSelectButton
            BackgroundColor({ props.player_color.color() })
            Pickable
            on(on_player_color_clicked)
        }
    }
}

#[derive(Component, FromTemplate, Clone)]
struct ButtonTextRef {
    entity: Entity,
}

#[derive(Component, Default, Clone)]
struct PlayerId(usize);

fn player_element(player_id: usize, player_info: PlayerInfo) -> Box<dyn Scene> {
    assert!(player_id <= NB_PLAYER_COLORS && player_id != 0);

    let scene = bsn! {
        Node {
            align_items: AlignItems::Center,

            column_gap: px(3.0)
        }
        PlayerId(player_id)

        Children [
            label(format!("Player {player_id}")),
            @ColorSelectButton {
                player_id,
                @player_color: {player_info.player_color}
            },

            @FeathersButton {
                @caption: bsn! {
                    #ButtonText
                    Text::new(format!("{}", if player_info.bot {"Bot"} else {"Player"}))
                    ThemedText
                }
            }
            ButtonTextRef {
                entity: #ButtonText
            }
            on(on_bot_button_clicked)
        ]
    };

    Box::new(scene)
}

fn building_assignements(game_preparation_info: GamePreparationInfo) -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(3.0),

            border: px(2.0),
            border_radius: px(2.0),
            padding: px(5.0),
        }
        BorderColor::all(WHITE)

        Children [
            {building_assignement_elements(game_preparation_info)}
        ]
    }
}

fn building_assignement_elements(
    game_preparation_info: GamePreparationInfo,
) -> Vec<Box<dyn Scene>> {
    let mut elements = Vec::new();

    let mut main_building_ids = game_preparation_info
        .building_assignements
        .keys()
        .copied()
        .collect::<Vec<_>>();

    main_building_ids.sort();

    for main_building_id in main_building_ids {
        let element = building_assignement_element(main_building_id);

        elements.push(element);
    }

    elements
}

#[derive(SceneComponent, Default, Clone)]
struct BuildingAssignementButton {
    building_id: usize,
}

impl BuildingAssignementButton {
    fn scene() -> impl Scene {
        bsn! {
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                width: px(20.0),
                height: px(20.0),

                border: px(2.0),
                border_radius: px(2.0),
            }
            Pickable
            BorderColor::all(PlayerColor::Neutral.color())

            Children [
                label("-")
            ]
            on(on_building_assignement_button_clicked)
        }
    }
}

fn building_assignement_element(building_id: usize) -> Box<dyn Scene> {
    let scene = bsn! {
        Node {
            column_gap: px(5.0),
        }

        Children [
            label(format!("Main building {building_id}")),
            @BuildingAssignementButton { building_id },
        ]
    };

    Box::new(scene)
}

fn map_preview_image(image: Handle<Image>) -> impl Scene {
    bsn! {
        Node {
            justify_self: JustifySelf::End,
            align_self: AlignSelf::Center,

            width: px(MAP_PREVIEW_IMAGE_SIZE.x as f32),
            height: px(MAP_PREVIEW_IMAGE_SIZE.y as f32),

            border: px(3.0),
        }
        BorderColor::all(Color::srgb(1.0, 1.0, 1.0))

        ImageNode {
            image
        }
    }
}

fn bottom_bar() -> impl Scene {
    bsn! {
        Node {
            justify_self: JustifySelf::End,
            align_self: AlignSelf::Stretch,

            justify_content: JustifyContent::SpaceBetween,

            border: px(2.0),
            border_radius: px(2.0),
            padding: px(5.0),
        }
        BorderColor::all(WHITE)

        Children [
            main_menu_button(),
            play_button(),
        ]
    }
}

fn main_menu_button() -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new("Main Menu")
                ThemedText
            }
        }
        on(|_: On<Activate>, mut next_state: ResMut<NextState<MenuState>>| next_state.set(MenuState::MainMenu))
    }
}

#[derive(SceneComponent, Default, Clone)]
struct PlayButton;

impl PlayButton {
    fn scene() -> impl Scene {
        bsn! {
            @FeathersButton {
                @caption: bsn! {
                    Text::new("Play")
                    ThemedText
                }
            }
            InteractionDisabled
            on(play_button_clicked)
        }
    }
}

fn play_button() -> impl Scene {
    bsn! {
        @PlayButton
    }
}

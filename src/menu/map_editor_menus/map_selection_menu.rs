use bevy::{
    color::palettes::css::WHITE,
    feathers::{
        controls::{FeathersButton, FeathersTextInput},
        theme::ThemedText,
    },
    prelude::*,
    ui::InteractionDisabled,
};

use crate::{
    map::{MapCollection, MapInfo, MapName},
    map_editor::MapEditorState,
    menu::map_editor_menus::menu_backend::{
        create_map_button_clicked, main_menu_button_clicked, on_button_delete, on_button_edit,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(MapEditorState::MapSelection),
        on_enter_map_selection,
    );
    app.add_systems(
        OnExit(MapEditorState::MapSelection),
        despawn_map_selection_menu,
    );

    app.add_observer(respawn_map_selection_menu);
}

#[derive(Component, Default, Clone)]
struct MapSelectionMenu;

#[derive(Event)]
pub struct RespawnMapSelectionMenu;

fn on_enter_map_selection(mut commands: Commands) {
    commands.trigger(RespawnMapSelectionMenu);
}

fn respawn_map_selection_menu(
    _: On<RespawnMapSelectionMenu>,
    mut commands: Commands,
    maps: Res<MapCollection>,
    map_selection_menu: Query<Entity, With<MapSelectionMenu>>,
) {
    for entity in map_selection_menu {
        commands.entity(entity).despawn();
    }

    let mut map_infos = maps.map_infos();

    map_infos.sort_by(|info_1, info_2| info_1.map_name().cmp(&info_2.map_name()));

    commands.spawn_scene(bsn! {
        MapSelectionMenu

        Node {
            flex_direction: FlexDirection::Column,

            border: px(3.0),
            padding: px(5.0),

            min_width: px(200.0),

            row_gap: px(10.0)
        }
        BorderColor::all(WHITE)

        Children [
            map_elements(map_infos),
            new_map(),
            main_menu_button(),
        ]
    });
}

fn despawn_map_selection_menu(
    mut commands: Commands,
    map_list: Single<Entity, With<MapSelectionMenu>>,
) {
    commands.entity(*map_list).despawn();
}

#[derive(Component, Default, Clone)]
pub struct MapNameTextInput;

#[derive(Component, Default, Clone)]
pub struct CreateMapButton;

fn new_map() -> impl Scene {
    bsn! {
        Node {
            column_gap: px(10.0),
        }

        Children [
            (
                MapNameTextInput
                @FeathersTextInput
            ),
            (
                CreateMapButton
                InteractionDisabled
                @FeathersButton {
                    @caption: bsn! {
                        Text::new("Create map")
                        ThemedText
                    }
                }
                on(create_map_button_clicked)
            )
        ]
    }
}

fn main_menu_button() -> impl Scene {
    bsn! {
        Node

        @FeathersButton {
            @caption: bsn! {
                Text::new("Main Menu")
                ThemedText
            }
        }
        on(main_menu_button_clicked)
    }
}

fn map_elements(map_infos: Vec<MapInfo>) -> impl Scene {
    let mut map_elements = Vec::new();

    for map_info in map_infos {
        let map_element = map_element(map_info);
        map_elements.push(map_element);
    }

    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(3.0),
            border: px(2.0),
            padding: px(2.0),
        }
        BorderColor::all(WHITE)

        Children [
            {map_elements}
        ]
    }
}

fn map_element(map_info: MapInfo) -> Box<dyn Scene> {
    let map_name = MapName::new(map_info.map_name());

    let scene = bsn! {
        Node {
            column_gap: px(5.0)
        }

        template_value(map_name.clone())

        Children [
            Text::new(map_name.name())
            ThemedText,

            @FeathersButton {
                @caption: bsn! {
                    Text::new("Edit")
                    ThemedText
                }
            }
            on(on_button_edit),

            @FeathersButton {
                @caption: bsn! {
                    Text::new("Delete")
                    ThemedText
                }
            }
            on(on_button_delete),
        ]
    };

    Box::new(scene)
}

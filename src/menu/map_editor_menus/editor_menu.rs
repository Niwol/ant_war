use bevy::{
    color::palettes::css::{BLACK, WHITE},
    feathers::{
        controls::{
            FeathersButton, FeathersNumberInput, NumberFormat, NumberInputValue, UpdateNumberInput,
        },
        display::label,
        theme::ThemedText,
    },
    prelude::*,
    ui_widgets::Activate,
};

use crate::{
    AppState,
    cursor::Cursor,
    game::building::building_type::BuildingType,
    map_editor::{
        MapEditorState,
        editor::{self, AddBuilding, CurrentMap, SaveMap},
    },
    world_grid::coord::Coord,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(MapEditorState::InEditor), spawn_map_editor_menu);
    app.add_systems(OnExit(MapEditorState::InEditor), despawn_map_editor_ui);

    app.add_systems(Update, update_coord_text);

    app.add_observer(on_map_ui_spawned);
}

#[derive(Component, Default, Clone)]
enum MapEditorMenus {
    #[default]
    LeftPanel,
    BottomBar,
}

#[derive(Event)]
struct MapUiSpawned;

#[derive(Resource)]
pub struct MapUiEntities {
    pub map_width: Entity,
    pub map_height: Entity,
}

#[derive(Component, Default, Clone)]
pub enum MapUiEntity {
    #[default]
    MapWidthTextInput,
    MapHeightTextInput,
}

#[derive(Component, Default, Clone)]
struct CoordText;

fn spawn_map_editor_menu(mut commands: Commands, current_map: Res<CurrentMap>) {
    commands.spawn_scene_list(bsn_list!(
        left_panel(&current_map.map.map_name()),
        bottom_bar()
    ));

    commands.trigger(MapUiSpawned);
}

fn despawn_map_editor_ui(
    mut commands: Commands,
    map_editor_ui: Query<Entity, With<MapEditorMenus>>,
) {
    for entity in map_editor_ui {
        commands.entity(entity).despawn();
    }
}

fn left_panel(map_name: impl Into<String>) -> impl Scene {
    bsn! {
        template_value(MapEditorMenus::LeftPanel)
        Node {
            align_self: AlignSelf::FlexStart,
            justify_self: JustifySelf::Start,

            flex_direction: FlexDirection::Column,

            min_width: percent(15.0),

            border: px(3.0),
            border_radius: px(5.0)
            padding: px(5.0),

            row_gap: px(20.0),
        }
        BorderColor::all(WHITE)

        Children [
            map_name_label(map_name),
            map_size_text_input(),
            add_building_buttons(),
            save_button(),
            main_menu_button()
        ]
    }
}

fn bottom_bar() -> impl Scene {
    bsn! {
        template_value(MapEditorMenus::BottomBar)
        Node {
            align_self: AlignSelf::FlexEnd,
            justify_self: JustifySelf::Stretch,

            border: px(3.0),
            border_radius: px(5.0),
            padding: px(5.0),
            margin: px(5.0),
        }
        BorderColor::all(WHITE)
        BackgroundColor(BLACK)

        Children [
            CoordText
            label("Coord: -")
        ]
    }
}

fn map_name_label(map_name: impl Into<String>) -> impl Scene {
    bsn! {
        label(map_name)
    }
}

fn map_size_text_input() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,

            row_gap: px(5.0),
        }

        Children [
            label("Map size"),

            template_value(MapUiEntity::MapWidthTextInput)
            @FeathersNumberInput {
                @label_text: "Width: ",
                @number_format: NumberFormat::I32,
            }
            on(editor::on_update_map_width),

            template_value(MapUiEntity::MapHeightTextInput)
            @FeathersNumberInput {
                @label_text: "Height: ",
                @number_format: NumberFormat::I32,
            }
            on(editor::on_update_map_height)
        ]
    }
}

fn add_building_buttons() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,

            row_gap: px(5.0),
        }

        Children [
            label("Add building"),

            add_building_button(BuildingType::HeadQuarter { index: 0 }),
            add_building_button(BuildingType::House),
            add_building_button(BuildingType::Tower),
            add_building_button(BuildingType::Casern),
            add_building_button(BuildingType::Walls),
        ]
    }
}

fn add_building_button(building_type: BuildingType) -> impl Scene {
    let button_text = building_type.name();

    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new(button_text)
                ThemedText
            }
        }
        on(move |_: On<Activate>, mut commands: Commands| {
            commands.trigger(AddBuilding(building_type));
        })
    }
}

fn save_button() -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new("Save map")
                ThemedText
            }
        }

        on(save_button_clicked)
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

        on(main_menu_clicked)
    }
}

fn save_button_clicked(_: On<Activate>, mut commands: Commands) {
    commands.trigger(SaveMap);
}

fn main_menu_clicked(_: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}

fn on_map_ui_spawned(
    _: On<MapUiSpawned>,
    mut commands: Commands,
    map_ui_entities_query: Query<(Entity, &MapUiEntity)>,
    current_map: Res<CurrentMap>,
) {
    let mut map_ui_entities = MapUiEntities {
        map_width: Entity::PLACEHOLDER,
        map_height: Entity::PLACEHOLDER,
    };

    for (entity, map_entity) in &map_ui_entities_query {
        match map_entity {
            MapUiEntity::MapWidthTextInput => map_ui_entities.map_width = entity,
            MapUiEntity::MapHeightTextInput => map_ui_entities.map_height = entity,
        }
    }

    let map = &current_map.map;

    let map_size = map.size();

    commands.trigger(UpdateNumberInput {
        entity: map_ui_entities.map_width,
        value: NumberInputValue::I32(map_size.x as i32),
    });

    commands.trigger(UpdateNumberInput {
        entity: map_ui_entities.map_height,
        value: NumberInputValue::I32(map_size.y as i32),
    });

    commands.insert_resource(map_ui_entities);
}

fn update_coord_text(cursor: Res<Cursor>, mut coord_text: Single<&mut Text, With<CoordText>>) {
    let text = match cursor.world_pos() {
        Some(pos) => {
            let coord = Coord::from_world(pos);

            format!("Coord: ({}, {})", coord.0.x, coord.0.y)
        }
        None => format!("Coord: -"),
    };

    coord_text.0 = text;
}

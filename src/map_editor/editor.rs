use bevy::{
    prelude::*,
    ui_widgets::{Activate, ValueChange},
};

use crate::{
    MainCamera,
    game::building::BuildingType,
    map::{self, Map, MapAccess, MapCollection, MapInfo},
    map_editor::{
        MapEditorState,
        editor::{
            background_sprite::BackgroundSprite,
            editor_building::{
                DespawnEditorBuilding, EditorBuilding, EditorBuildingPlugin, SpawnEditorBuilding,
            },
            invalid_location::InvalidLocationPlugin,
            preview_building::PreviewBuildingPlugin,
        },
    },
    world_grid::{WorldGrid, grid_transform::GridTransform},
};

pub mod background_sprite;
pub mod editor_building;
pub mod invalid_location;
pub mod preview_building;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PreviewBuildingPlugin,
            InvalidLocationPlugin,
            EditorBuildingPlugin,
        ));

        app.add_systems(
            Update,
            load_map.run_if(in_state(MapEditorState::LoadingMap)),
        );

        app.add_systems(OnEnter(MapEditorState::InEditor), spawn_map);

        app.add_observer(on_load_map);

        app.add_observer(save_map);
        app.add_observer(on_resize_map);
    }
}

#[derive(Event)]
pub struct LoadMap {
    pub map_name: String,
}

#[derive(Component, Default, Clone, Copy)]
struct DragBuilding {
    origin_transform: GridTransform,
}

#[derive(Resource)]
pub struct CurrentMap {
    pub map: Map,
    pub handle: Option<Handle<Map>>,
}

#[derive(Resource)]
struct MapToLoad(Handle<Map>);

#[derive(Resource)]
pub struct EditorState {
    map_size: UVec2,
    selected_building: Option<Entity>,
    add_building_preveiw: Option<Entity>,
}

fn on_load_map(
    load_map: On<LoadMap>,
    mut commands: Commands,
    map_collection: Res<MapCollection>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<MapEditorState>>,
) {
    let map_info = map_collection.map_info(load_map.map_name.clone()).unwrap();

    let map_handle = match map_info._map_access() {
        MapAccess::Handle(handle) => handle.clone(),
        MapAccess::AssetPath(asset_path) => asset_server.load(asset_path),
    };

    commands.insert_resource(MapToLoad(map_handle));
    next_state.set(MapEditorState::LoadingMap);
}

fn load_map(
    map_to_load: Res<MapToLoad>,
    mut commands: Commands,
    maps: Res<Assets<Map>>,
    mut next_state: ResMut<NextState<MapEditorState>>,
) {
    let map = maps.get(&map_to_load.0);

    if let Some(map) = map {
        commands.insert_resource(CurrentMap {
            map: map.clone(),
            handle: Some(map_to_load.0.clone()),
        });

        commands.remove_resource::<MapToLoad>();
        next_state.set(MapEditorState::InEditor);
    }
}

fn spawn_map(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    mut camera_transform: Single<&mut Transform, With<MainCamera>>,
) {
    let map = &current_map.map;
    let world_grid = WorldGrid::new(map.size());

    let map_center = world_grid.center();

    camera_transform.translation = Vec3 {
        x: map_center.x,
        y: map_center.y,
        z: 0.0,
    };

    commands.insert_resource(world_grid);
    for (building_id, building_info) in map.building_infos() {
        commands.trigger(SpawnEditorBuilding {
            building_id,
            building_info,
        });
    }

    commands.spawn_scene(bsn! {
        @BackgroundSprite {
            @size: {map.size_world()},
            @pos: map_center,
        }
    });

    commands.insert_resource(EditorState {
        map_size: map.size(),
        selected_building: None,
        add_building_preveiw: None,
    });
}

#[derive(Event)]
pub struct SaveMap;

fn save_map(
    _: On<SaveMap>,
    mut commands: Commands,
    mut maps: ResMut<Assets<Map>>,
    current_map: Res<CurrentMap>,
) {
    let map_handle = match &current_map.handle {
        Some(handle) => {
            let mut map = maps.get_mut(handle).unwrap();
            *map = current_map.map.clone();

            handle.clone()
        }
        None => maps.add(current_map.map.clone()),
    };

    let map_info = MapInfo::new(current_map.map.map_name(), MapAccess::Handle(map_handle));

    commands.trigger(map::map_io::SaveMap { map_info });
}

#[derive(Event)]
pub struct AddBuilding(pub BuildingType);

#[derive(Event)]
struct ResizeMap {
    new_size: UVec2,
}

pub fn on_add_main_building_clicked(
    _: On<Activate>,
    mut commands: Commands,
    current_map: Res<CurrentMap>,
) {
    let index = current_map.map.next_main_building_index();

    let building_type = BuildingType::MainBuilding { index };
    commands.trigger(AddBuilding(building_type));
}

pub fn on_update_map_width(
    update: On<ValueChange<i32>>,
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
) {
    if update.is_final {
        if update.value <= 0 {
            return;
        }

        let mut grid_size = world_grid.grid_size();
        grid_size.x = update.value as u32;

        commands.trigger(ResizeMap {
            new_size: grid_size,
        });
    }
}

pub fn on_update_map_height(
    update: On<ValueChange<i32>>,
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
) {
    if update.is_final {
        if update.value <= 0 {
            return;
        }

        let mut grid_size = world_grid.grid_size();
        grid_size.y = update.value as u32;

        commands.trigger(ResizeMap {
            new_size: grid_size,
        });
    }
}

fn on_resize_map(
    resize: On<ResizeMap>,
    mut commands: Commands,
    mut world_grid: ResMut<WorldGrid>,
    mut editor_state: ResMut<EditorState>,
    editor_buildings: Query<&EditorBuilding>,
) {
    let entities = world_grid.resize(resize.new_size);
    editor_state.map_size = resize.new_size;

    for entity in entities {
        if editor_buildings.get(entity).is_ok() {
            commands.trigger(DespawnEditorBuilding { entity });
        }
    }
}

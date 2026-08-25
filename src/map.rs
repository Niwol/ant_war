use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    game::building::{BuildingId, BuildingType},
    manifest::Manifest,
    world_grid::{CELL_SIZE, grid_transform::GridTransform},
};

pub mod map_io;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(map_io::plugin);

        app.add_sub_state::<MapLoadingState>();

        app.add_systems(Update, load_maps.run_if(in_state(MapLoadingState::Loading)));
    }
}

#[derive(Component, Default, Clone)]
pub struct MapName(String);

impl MapName {
    pub fn new(name: impl ToString) -> Self {
        Self(name.to_string())
    }

    pub fn name(&self) -> String {
        self.0.clone()
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub struct BuildingInfo {
    pub building_type: BuildingType,
    pub grid_transform: GridTransform,
}

#[derive(Asset, TypePath, Deserialize, Serialize, Clone)]
pub struct Map {
    map_name: String,
    map_size: UVec2,
    max_players: usize,
    building_infos: HashMap<BuildingId, BuildingInfo>,
}

impl Map {
    pub fn new(map_name: impl Into<String>, map_size: UVec2, max_players: usize) -> Self {
        Self {
            map_name: map_name.into(),
            map_size,
            max_players,
            building_infos: HashMap::new(),
        }
    }

    pub fn _set_size(&mut self, size: UVec2) {
        self.map_size = size;
    }

    pub fn _set_max_players(&mut self, max_players: usize) {
        self.max_players = max_players;
    }

    pub fn add_building(&mut self, building_info: BuildingInfo) -> BuildingId {
        let building_id = self.next_building_id();

        self.building_infos.insert(building_id, building_info);

        building_id
    }

    pub fn remove_building(&mut self, building_id: BuildingId) {
        self.building_infos.remove(&building_id);
    }

    pub fn map_name(&self) -> String {
        self.map_name.clone()
    }

    pub fn building_infos(&self) -> Vec<BuildingInfo> {
        self.building_infos.values().copied().collect::<Vec<_>>()
    }

    pub fn size(&self) -> UVec2 {
        self.map_size
    }

    pub fn size_world(&self) -> Vec2 {
        Vec2 {
            x: CELL_SIZE * self.map_size.x as f32,
            y: CELL_SIZE * self.map_size.y as f32,
        }
    }

    pub fn max_players(&self) -> usize {
        self.max_players
    }

    pub fn get_building_info_mut(&mut self, building_id: BuildingId) -> Option<&mut BuildingInfo> {
        self.building_infos.get_mut(&building_id)
    }

    fn next_building_id(&self) -> BuildingId {
        let mut id = BuildingId::new(0);

        while self.building_infos.contains_key(&id) {
            let new_id = id.id() + 1;
            id = BuildingId::new(new_id);
        }

        id
    }
}

#[derive(SubStates, Default, Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[source(AppState = AppState::Initialisation)]
pub enum MapLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Clone)]
pub enum MapAccess {
    Handle(Handle<Map>),
    AssetPath(String),
}

#[derive(Clone)]
pub struct MapInfo {
    map_name: String,
    map_access: MapAccess,
}

impl MapInfo {
    pub fn new(map_name: String, map_access: MapAccess) -> Self {
        Self {
            map_name,
            map_access,
        }
    }

    pub fn map_name(&self) -> String {
        self.map_name.clone()
    }

    pub fn path(&self) -> String {
        format!("maps/{}.map", self.map_name)
    }

    pub fn _map_access(&self) -> MapAccess {
        self.map_access.clone()
    }
}

#[derive(Resource)]
pub struct MapCollection(HashMap<String, MapInfo>);

impl MapCollection {
    pub fn map_infos(&self) -> Vec<MapInfo> {
        self.0.values().cloned().collect::<Vec<_>>()
    }

    pub fn map_info(&self, map_name: String) -> Option<MapInfo> {
        self.0.get(&map_name).cloned()
    }

    pub fn _add_map(&mut self, map_info: MapInfo) {
        let map_name = map_info.map_name.clone();
        self.0.insert(map_name, map_info);
    }

    pub fn remove(&mut self, map_name: String) {
        self.0.remove(&map_name);
    }
}

fn load_maps(
    mut commands: Commands,
    manifest: Res<Manifest>,
    mut next_state: ResMut<NextState<MapLoadingState>>,
) {
    if !manifest.loaded() {
        return;
    }

    let map_files = manifest.map_files();

    let mut maps = MapCollection(HashMap::new());

    for map_file in map_files {
        let map_name = map_file.clone();
        let map_name = map_name.split("/").last().unwrap();
        let map_name = map_name.split(".").next().unwrap();

        let map_info = MapInfo {
            map_name: map_name.to_string(),
            map_access: MapAccess::AssetPath(map_file),
        };

        maps.0.insert(map_name.to_string(), map_info);
    }

    commands.insert_resource(maps);
    next_state.set(MapLoadingState::Loaded);
}

use bevy::{ecs::system::SystemParam, feathers::FeathersPlugins, prelude::*};

use crate::{
    cursor::CursorPlugin,
    game::GamePlugin,
    manifest::{ManifestLoadingState, ManifestPlugin},
    map::{MapLoadingState, MapPlugin},
    map_editor::MapEditorPlugin,
    menu::MenuPlugin,
    ui::UiPlugin,
    world_grid::GridPlugin,
};

mod cursor;
mod game;
mod manifest;
mod map;
mod map_editor;
mod menu;
mod ui;
mod world_grid;

pub struct AntWarPlugin;
impl Plugin for AntWarPlugin {
    fn build(&self, app: &mut App) {
        // Bevy plugins
        app.add_plugins((DefaultPlugins, FeathersPlugins));

        // Crate plugins
        app.add_plugins((
            ManifestPlugin,
            MapPlugin,
            CursorPlugin,
            GamePlugin,
            GridPlugin,
            MapEditorPlugin,
            MenuPlugin,
            UiPlugin,
        ));

        app.init_state::<AppState>();

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            initialize.run_if(in_state(AppState::Initialisation)),
        );
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    Initialisation,
    MainMenu,
    InGame,
    InMapEditor,
}

#[derive(Component, Default, Clone)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        MainCamera
        Camera2d
    });
}

#[derive(SystemParam)]
struct InitialisationStates<'w> {
    manifest_state: Res<'w, State<ManifestLoadingState>>,
    map_state: Res<'w, State<MapLoadingState>>,
}

impl<'w> InitialisationStates<'w> {
    fn initialized(&self) -> bool {
        match **self.manifest_state {
            ManifestLoadingState::Loading => return false,
            ManifestLoadingState::Loaded => (),
        };

        match **self.map_state {
            MapLoadingState::Loading => return false,
            MapLoadingState::Loaded => (),
        };

        println!("Initialized");

        true
    }
}

fn initialize(mut next_state: ResMut<NextState<AppState>>, init_states: InitialisationStates) {
    if init_states.initialized() {
        next_state.set(AppState::MainMenu);
    }
}

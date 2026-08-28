use bevy::{
    feathers::{dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

use crate::{
    AppState,
    menu::{
        game_preparation_menu::GamePreparationMenuPlugin, in_game_menus::InGameMenusPlugin,
        main_menu::MainMenu, map_editor_menus::MapEditorMenusPlugin,
    },
};

pub mod game_preparation_menu;
pub mod in_game_menus;
mod main_menu;
pub mod map_editor_menus;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GamePreparationMenuPlugin,
            InGameMenusPlugin,
            MapEditorMenusPlugin,
        ));

        app.insert_resource(UiTheme(create_dark_theme()));
        app.add_sub_state::<MenuState>();

        app.add_systems(OnEnter(MenuState::MainMenu), spawn_main_menu);
        app.add_systems(OnExit(MenuState::MainMenu), despawn_main_menu);
    }
}

#[derive(SubStates, Hash, Debug, PartialEq, Eq, Default, Clone, Copy)]
#[source(AppState = AppState::MainMenu)]
pub enum MenuState {
    #[default]
    MainMenu,
    GamePreparation,
}

fn spawn_main_menu(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resolution.set(1000.0, 800.0);
    commands.spawn_scene(main_menu::main_menu());
}

fn despawn_main_menu(mut commands: Commands, main_menu: Single<Entity, With<MainMenu>>) {
    commands.entity(*main_menu).despawn();
}

use bevy::{
    feathers::{dark_theme::create_dark_theme, theme::UiTheme},
    prelude::*,
};

use crate::menu::{
    game_preparation_menu::GamePreparationMenuPlugin, in_game_menus::InGameMenusPlugin,
    main_menu::MainMenuPlugin, map_editor_menus::MapEditorMenusPlugin,
};

pub mod game_preparation_menu;
pub mod in_game_menus;
mod main_menu;
pub mod map_editor_menus;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MainMenuPlugin,
            GamePreparationMenuPlugin,
            InGameMenusPlugin,
            MapEditorMenusPlugin,
        ));

        app.insert_resource(UiTheme(create_dark_theme()));
    }
}

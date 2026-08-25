use bevy::prelude::*;

use crate::menu::in_game_menus::{game_over_menu::GameOverMenuPlugin, pause_menu::PauseMenuPlugin};

pub mod game_over_menu;
pub mod pause_menu;

pub struct InGameMenusPlugin;
impl Plugin for InGameMenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PauseMenuPlugin, GameOverMenuPlugin));
    }
}

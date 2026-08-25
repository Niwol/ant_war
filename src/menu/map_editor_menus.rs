use bevy::prelude::*;

pub mod editor_menu;
pub mod map_selection_menu;
mod menu_backend;

pub struct MapEditorMenusPlugin;
impl Plugin for MapEditorMenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            map_selection_menu::plugin,
            editor_menu::plugin,
            menu_backend::plugin,
        ));
    }
}

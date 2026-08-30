use bevy::prelude::*;

use crate::ui::selection_rect::SelectionRectPlugin;

pub mod selection_rect;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectionRectPlugin);
    }
}

use bevy::prelude::*;

use crate::ui::{bottom_panel::BottomPanelPlugin, selection_rect::SelectionRectPlugin};

pub mod bottom_panel;
pub mod selection_rect;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SelectionRectPlugin, BottomPanelPlugin));
    }
}

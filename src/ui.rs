use bevy::prelude::*;

use crate::ui::{control_helper::ControlHelperPlugin, selection_rect::SelectionRectPlugin};

mod control_helper;
pub mod selection_rect;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SelectionRectPlugin, ControlHelperPlugin));
    }
}

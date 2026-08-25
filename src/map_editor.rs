use bevy::prelude::*;

use crate::{AppState, map_editor::editor::EditorPlugin, world_grid::WorldGrid};

pub mod editor;

pub struct MapEditorPlugin;
impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MapEditorState>();

        app.add_plugins(EditorPlugin);

        app.add_systems(OnExit(AppState::InMapEditor), cleanup_map_editor);
    }
}

#[derive(SubStates, Hash, PartialEq, Eq, Clone, Copy, Debug, Default)]
#[source(AppState = AppState::InMapEditor)]
pub enum MapEditorState {
    #[default]
    MapSelection,
    LoadingMap,
    InEditor,
}

#[derive(Component, Default, Clone)]
struct MapEditorEntity;

fn cleanup_map_editor(
    mut commands: Commands,
    map_editor_entities: Query<Entity, With<MapEditorEntity>>,
) {
    for entity in &map_editor_entities {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<WorldGrid>();
}

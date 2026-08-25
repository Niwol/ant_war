use std::path::Path;

use bevy::{
    asset::{
        AssetPath,
        saver::{SavedAsset, save_using_saver},
    },
    prelude::*,
    tasks::IoTaskPool,
    text::EditableText,
    ui::InteractionDisabled,
    ui_widgets::Activate,
};

use crate::{
    AppState,
    manifest::{Manifest, ManifestAsset, ManifestAssetSaver},
    map::{Map, MapCollection, MapName},
    map_editor::{
        MapEditorState,
        editor::{CurrentMap, LoadMap},
    },
    menu::map_editor_menus::map_selection_menu::{
        CreateMapButton, MapNameTextInput, RespawnMapSelectionMenu,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_create_map_button_interactibility);
}

pub fn on_button_edit(
    activation: On<Activate>,
    mut commands: Commands,
    map_names: Query<&MapName>,
    children: Query<&ChildOf>,
) {
    let child_of = children.get(activation.entity).unwrap();
    let map_name = map_names.get(child_of.0).unwrap().name();

    commands.trigger(LoadMap { map_name });
}

pub fn on_button_delete(
    activation: On<Activate>,
    mut commands: Commands,
    mut maps: ResMut<MapCollection>,
    map_names: Query<&MapName>,
    children: Query<&ChildOf>,
    mut manifest: ResMut<Manifest>,
    asset_server: Res<AssetServer>,
) {
    let child_of = children.get(activation.entity).unwrap();
    let map_name = map_names.get(child_of.0).unwrap();

    let Some(map_info) = maps.map_info(map_name.name()) else {
        return;
    };

    maps.remove(map_name.name());

    manifest.remove_file(map_info.path());

    let files = manifest.files().clone();
    let mut manifest_content = String::new();

    for file in files {
        let s = format!("{file}\n");
        manifest_content.push_str(&s);
    }

    let manifest_asset = ManifestAsset::new(manifest_content);
    let asset_server_manifest = asset_server.clone();

    IoTaskPool::get()
        .spawn(async move {
            match save_using_saver(
                asset_server_manifest.clone(),
                &ManifestAssetSaver,
                &AssetPath::from_path(Path::new("manifest.txt")),
                SavedAsset::from_asset(&manifest_asset),
                &(),
            )
            .await
            {
                Ok(()) => info!("Manifest saved"),
                Err(err) => error!("Failed to save asset: {err}"),
            }
        })
        .detach();

    commands.trigger(RespawnMapSelectionMenu);
}

pub fn create_map_button_clicked(
    _: On<Activate>,
    mut commands: Commands,
    map_name_text: Single<&EditableText, With<MapNameTextInput>>,
    mut next_state: ResMut<NextState<MapEditorState>>,
) {
    let map_name = map_name_text.editor.raw_text();
    let map = Map::new(map_name, UVec2::splat(20), 2);

    commands.insert_resource(CurrentMap { map, handle: None });
    next_state.set(MapEditorState::InEditor);
}

fn update_create_map_button_interactibility(
    mut commands: Commands,
    map_name_text: Single<&EditableText, (With<MapNameTextInput>, Changed<EditableText>)>,
    create_map_button: Single<Entity, With<CreateMapButton>>,
) {
    let text = map_name_text.editor.raw_text();
    if text.len() == 0 {
        commands
            .entity(*create_map_button)
            .insert(InteractionDisabled);
    } else {
        commands
            .entity(*create_map_button)
            .remove::<InteractionDisabled>();
    }
}

pub fn main_menu_button_clicked(_: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}

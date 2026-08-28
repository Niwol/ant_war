use std::{fs, io::ErrorKind, path::Path};

use bevy::{
    asset::{
        AssetLoader, AssetPath, AsyncWriteExt, LoadContext,
        io::{Reader, Writer},
        saver::{AssetSaver, SavedAsset, save_using_saver},
    },
    prelude::*,
    tasks::IoTaskPool,
};
use ron::ser::PrettyConfig;
use thiserror::Error;

use crate::{
    manifest::{Manifest, ManifestAsset, ManifestAssetSaver},
    map::{self, Map, MapAccess, MapCollection, MapInfo},
};

pub fn plugin(app: &mut App) {
    app.init_asset::<Map>();
    app.init_asset_loader::<MapAssetLoader>();

    app.add_observer(save_map);
    app.add_observer(delete_map);
}

#[derive(Default, TypePath)]
pub struct MapAssetLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum MapAssetLoaderError {
    #[error("Could not load asset {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse RON {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for MapAssetLoader {
    type Asset = Map;
    type Error = MapAssetLoaderError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let map_asset = ron::de::from_bytes::<Map>(&bytes)?;
        Ok(map_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}

#[derive(Clone, TypePath)]
pub struct MapAssetSaver;

impl AssetSaver for MapAssetSaver {
    type Asset = Map;
    type Settings = ();
    type OutputLoader = MapAssetLoader;
    type Error = BevyError;

    async fn save(
        &self,
        writer: &mut Writer,
        asset: SavedAsset<'_, '_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: AssetPath<'_>,
    ) -> Result<(), Self::Error> {
        let pretty_config = PrettyConfig::new()
            .compact_arrays(true)
            .compact_maps(true)
            .compact_structs(false);

        let serialised = ron::ser::to_string_pretty(&*asset, pretty_config)?;
        writer.write_all(serialised.as_bytes()).await?;

        Ok(())
    }
}

#[derive(Event)]
pub struct SaveMap {
    pub map_info: MapInfo,
}

#[derive(Event)]
pub struct DeleteMap {
    pub map_info: MapInfo,
}

fn save_map(
    save_map: On<SaveMap>,
    maps: Res<Assets<Map>>,
    mut map_collection: ResMut<MapCollection>,
    asset_server: Res<AssetServer>,
    mut manifest: ResMut<Manifest>,
) {
    let mut map_info = save_map.map_info.clone();
    let handle = match &map_info.map_access {
        map::MapAccess::Handle(handle) => handle.clone(),
        map::MapAccess::AssetPath(_) => unreachable!("The map access must be a handle"),
    };

    let map_name = map_info.map_name.clone();
    map_collection.0.insert(map_name, map_info.clone());

    #[cfg(not(target_family = "wasm"))]
    {
        let asset_path = map_info.path();
        map_info.map_access = MapAccess::AssetPath(asset_path);

        let map = maps.get(&handle).unwrap().clone();
        let asset_path = map_info.path();
        map_info.map_access = MapAccess::AssetPath(asset_path.clone());

        // Saving to disc
        // Map
        let asset_server_map = asset_server.clone();
        IoTaskPool::get()
            .spawn(async move {
                match save_using_saver(
                    asset_server_map.clone(),
                    &MapAssetSaver,
                    &AssetPath::from_path(Path::new(&asset_path)),
                    SavedAsset::from_asset(&map),
                    &(),
                )
                .await
                {
                    Ok(()) => info!("Map saved"),
                    Err(err) => error!("Failed to save asset: {err}"),
                }
            })
            .detach();

        // Manifest
        let asset_server_manifest = asset_server.clone();

        manifest.insert_file(map_info.path());

        let mut files = manifest.files().iter().collect::<Vec<_>>();
        files.sort();
        let mut manifest_content = String::new();

        for file in files {
            let s = format!("{file}\n");
            manifest_content.push_str(&s);
        }

        let manifest_asset = ManifestAsset::new(manifest_content);

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
    }
}

fn delete_map(
    delete_map: On<DeleteMap>,
    mut map_collection: ResMut<MapCollection>,
    mut manifest: ResMut<Manifest>,
    asset_server: Res<AssetServer>,
) {
    let map_info = delete_map.map_info.clone();
    map_collection.remove(map_info.map_name());

    #[cfg(not(target_family = "wasm"))]
    {
        let file_path = map_info.path();

        manifest.remove_file(file_path.clone());
        let mut files = manifest.files().iter().collect::<Vec<_>>();
        files.sort();
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

        let file_path = format!("assets/{}", file_path);
        let meta_file_path = format!("{file_path}.meta");

        match fs::remove_file(Path::new(&file_path)) {
            Ok(_) => info!("File deleted"),
            Err(err) => warn!("Could not delete file: {err}"),
        }

        match fs::remove_file(Path::new(&meta_file_path)) {
            Ok(_) => info!("Meta file deleted"),
            Err(err) => {
                if err.kind() != ErrorKind::NotFound {
                    warn!("Could not delete file: {err}");
                }
            }
        }
    }
}

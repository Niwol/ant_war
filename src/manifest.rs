use bevy::{
    asset::{
        AssetLoader, AssetPath, AsyncReadExt, AsyncWriteExt, LoadContext,
        io::{Reader, Writer},
        saver::{AssetSaver, SavedAsset},
    },
    platform::collections::HashSet,
    prelude::*,
};

use thiserror::Error;

use crate::AppState;

pub struct ManifestPlugin;
impl Plugin for ManifestPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ManifestAsset>();
        app.init_asset_loader::<ManifestAssetLoader>();

        app.add_sub_state::<ManifestLoadingState>();

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            load_manifest.run_if(in_state(ManifestLoadingState::Loading)),
        );
    }
}

#[derive(SubStates, Default, Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[source(AppState = AppState::Initialisation)]
pub enum ManifestLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource)]
pub struct Manifest {
    handle: Handle<ManifestAsset>,
    files: HashSet<String>,
    loaded: bool,
}

impl Manifest {
    pub fn files(&self) -> &HashSet<String> {
        &self.files
    }

    pub fn map_files(&self) -> Vec<String> {
        self.files
            .iter()
            .filter(|file| file.starts_with("maps/"))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn insert_file(&mut self, file: String) {
        self.files.insert(file);
    }

    pub fn remove_file(&mut self, file: String) {
        self.files.remove(&file);
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let manifest_asset = asset_server.load("manifest.txt");

    let manifest = Manifest {
        handle: manifest_asset,
        files: HashSet::new(),
        loaded: false,
    };

    commands.insert_resource(manifest);
}

fn load_manifest(
    mut manifest: ResMut<Manifest>,
    manifest_assets: Res<Assets<ManifestAsset>>,
    mut next_state: ResMut<NextState<ManifestLoadingState>>,
    mut update_count: Local<usize>,
) {
    let manifest_asset = manifest_assets.get(&manifest.handle);
    let Some(manifest_asset) = manifest_asset else {
        *update_count += 1;
        return;
    };

    for line in manifest_asset.content.lines() {
        if line.len() != 0 {
            manifest.files.insert(line.to_string());
        }
    }

    manifest.loaded = true;

    next_state.set(ManifestLoadingState::Loaded);
}

#[derive(Asset, TypePath)]
pub struct ManifestAsset {
    content: String,
}

impl ManifestAsset {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

#[derive(Default, TypePath)]
pub struct ManifestAssetLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ManifestAssetLoaderError {
    #[error("Could not load asset {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not parse RON {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for ManifestAssetLoader {
    type Asset = ManifestAsset;
    type Error = ManifestAssetLoaderError;
    type Settings = ();

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = String::new();
        reader.read_to_string(&mut content).await?;
        Ok(ManifestAsset { content })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

#[derive(Clone, TypePath)]
pub struct ManifestAssetSaver;

impl AssetSaver for ManifestAssetSaver {
    type Asset = ManifestAsset;
    type Settings = ();
    type OutputLoader = ManifestAssetLoader;
    type Error = BevyError;

    async fn save(
        &self,
        writer: &mut Writer,
        asset: SavedAsset<'_, '_, Self::Asset>,
        _settings: &Self::Settings,
        _asset_path: AssetPath<'_>,
    ) -> Result<(), Self::Error> {
        writer.write(&asset.content.as_bytes()).await?;

        Ok(())
    }
}

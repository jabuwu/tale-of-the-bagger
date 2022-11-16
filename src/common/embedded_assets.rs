use bevy::{
    asset::{AssetIo, AssetIoError, Metadata},
    prelude::*,
    utils::BoxedFuture,
};
use std::path::{Path, PathBuf};

use include_dir::{include_dir, Dir};

static ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

struct EmbeddedAssetIo(Box<dyn AssetIo>);

impl AssetIo for EmbeddedAssetIo {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        let file = ASSETS.get_entry(path).unwrap().as_file().unwrap();
        Box::pin(async move { Ok(file.contents().to_vec()) })
    }

    fn read_directory(
        &self,
        _path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        unimplemented!();
    }

    fn watch_path_for_changes(&self, _path: &Path) -> Result<(), AssetIoError> {
        Ok(())
    }

    fn watch_for_changes(&self) -> Result<(), AssetIoError> {
        Ok(())
    }

    fn get_metadata(&self, _path: &Path) -> Result<Metadata, AssetIoError> {
        unimplemented!();
    }
}

pub struct EmbeddedAssetIoPlugin;

impl Plugin for EmbeddedAssetIoPlugin {
    fn build(&self, app: &mut App) {
        let default_io = AssetPlugin::default().create_platform_default_asset_io();
        let asset_io = EmbeddedAssetIo(default_io);
        app.insert_resource(AssetServer::new(asset_io));
    }
}

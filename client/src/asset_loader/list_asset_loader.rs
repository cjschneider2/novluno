use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::utils::thiserror::Error;
use core_compat::parser::lst::parse_lst;
use crate::ListAsset;

#[derive(Default)]
pub struct ListAssetLoader;

/// Possible errors that can be produced by [`CustomAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ListAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load asset: {0}")]
    Core(#[from] core_compat::error::Error),
}

impl AssetLoader for ListAssetLoader {
    type Asset = ListAsset;
    type Settings = ();
    type Error = ListAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let list = parse_lst(&bytes, false)?;
            let mut list_asset = ListAsset{ items: vec![] };
            list_asset.items = list.items;
            Ok(list_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["lst"]
    }
}

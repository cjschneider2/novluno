use crate::asset_loader::AssetLoaderError;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use core_compat::entity::map::Map;
use core_compat::parser::rmm::parse_rmm;

#[derive(Default, Debug, TypePath, Asset)]
pub struct MapAsset(pub Map);

#[derive(Default, TypePath, Debug)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    type Asset = MapAsset;
    type Settings = ();
    type Error = AssetLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'_ mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let map = parse_rmm(&bytes)?;
        Ok(MapAsset(map))
    }

    fn extensions(&self) -> &[&str] {
        &["rmm"]
    }
}

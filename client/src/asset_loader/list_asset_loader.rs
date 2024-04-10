use crate::asset_loader::AssetLoaderError;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::TypePath;
use core_compat::entity::list_item::ListItem;
use core_compat::parser::lst::parse_lst;

#[derive(Default, TypePath, Asset, Debug)]
pub struct ListAsset {
    pub items: Vec<ListItem>,
}

impl ListAsset {
    pub fn get_item(&self, id: usize) -> Option<&ListItem> {
        self.items.iter().find(|i| i.id as usize == id)
    }
}

#[derive(Default, TypePath, Debug)]
pub struct ListAssetLoader;

impl AssetLoader for ListAssetLoader {
    type Asset = ListAsset;
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

        let list = parse_lst(&bytes, false)?;
        let mut list_asset = ListAsset { items: vec![] };
        list_asset.items = list.items;
        Ok(list_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["lst"]
    }
}

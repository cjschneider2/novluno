use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::*;
use core_compat::entity::rmd::Rmd;
use core_compat::entity::rmd_type::RmdType;
use core_compat::parser::rmd::parse_rmd;

use crate::asset_loader::AssetLoaderError;

#[derive(Asset, TypePath, Debug)]
pub struct RmdAsset(pub Rmd);

#[derive(Default, TypePath)]
pub struct RmdAssetLoader;

impl AssetLoader for RmdAssetLoader {
    type Asset = RmdAsset;
    type Settings = ();
    type Error = AssetLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'_ mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let path = load_context.path().path();
        let kind = rmd_type_from_path(path);
        let rmd = parse_rmd(kind, &bytes)?;
        Ok(RmdAsset(rmd))
    }

    fn extensions(&self) -> &[&str] {
        &["rmd"]
    }
}

fn rmd_type_from_path(path: &std::path::Path) -> RmdType {
    for component in path.components() {
        match component.as_os_str().to_str().unwrap_or("") {
            "Bul" => return RmdType::Bullet,
            "Ico" => return RmdType::Icon,
            "Chr" => return RmdType::Character,
            "Obj" => return RmdType::Object,
            "Tle" => return RmdType::Tile,
            _ => {}
        }
    }
    RmdType::Tile
}

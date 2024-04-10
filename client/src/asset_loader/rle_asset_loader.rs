use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, LoadContext, RenderAssetUsages};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use core_compat::parser::rle::parse_rle;

use crate::asset_loader::AssetLoaderError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteKind {
    Bullet,
    Icon,
    Character,
    Object,
    Tile,
    Interface,
}

#[derive(Debug)]
pub struct RleSprite {
    pub index: u32,
    pub width: i32,
    pub height: i32,
    pub x_off: i32,
    pub y_off: i32,
    pub image_raw: Vec<u8>,
}

#[derive(Asset, TypePath, Debug)]
pub struct RleAsset {
    pub file_num: u32,
    pub kind: SpriteKind,
    pub sprites: Vec<RleSprite>,
}

#[derive(Default, TypePath)]
pub struct RleAssetLoader;

impl AssetLoader for RleAssetLoader {
    type Asset = RleAsset;
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

        // AssetPath::path() gives us the underlying std::path::Path.
        let std_path = load_context.path().path();
        let kind = sprite_kind_from_path(std_path);
        let file_num = file_num_from_path(std_path);

        let resource_file = parse_rle(file_num, &bytes)?;
        let sprites = resource_file
            .resources
            .into_iter()
            .map(|r| RleSprite {
                index: r.index,
                width: r.width,
                height: r.height,
                x_off: r.offset_x,
                y_off: r.offset_y,
                image_raw: r.image_raw,
            })
            .collect();

        Ok(RleAsset {
            file_num,
            kind,
            sprites,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["rle"]
    }
}

// Derive SpriteKind from the containing folder name (Bul, Ico, Chr, Obj, Tle, Int).
fn sprite_kind_from_path(path: &std::path::Path) -> SpriteKind {
    for component in path.components() {
        match component.as_os_str().to_str().unwrap_or("") {
            "Bul" => return SpriteKind::Bullet,
            "Ico" => return SpriteKind::Icon,
            "Chr" => return SpriteKind::Character,
            "Obj" => return SpriteKind::Object,
            "Tle" => return SpriteKind::Tile,
            "Int" => return SpriteKind::Interface,
            _ => {}
        }
    }
    SpriteKind::Tile
}

// Parse the numeric suffix from a filename like "tle00001" → 1.
fn file_num_from_path(path: &std::path::Path) -> u32 {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.trim_start_matches(|c: char| c.is_alphabetic()))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

pub fn rle_to_image(sprite: &RleSprite) -> Option<Image> {
    if sprite.width <= 0 || sprite.height <= 0 {
        return None;
    }
    let mut image = Image::new(
        Extent3d {
            width: sprite.width as u32,
            height: sprite.height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        sprite.image_raw.clone(),
        // RLE pixels decoded to RGBA8; treat as sRGB since these are art assets.
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = bevy::image::ImageSampler::nearest();
    Some(image)
}

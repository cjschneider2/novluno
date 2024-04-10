use std::collections::HashMap;

use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use core_compat::entity::entry::Entry;

use crate::asset_loader::rle_asset_loader::{rle_to_image, RleAsset, SpriteKind};

pub struct CachedSprite {
    pub handle: Handle<Image>,
    pub x_off: i32,
    pub y_off: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Resource, Default)]
pub struct SpriteCache {
    cache: HashMap<(SpriteKind, Entry), CachedSprite>,
}

impl SpriteCache {
    pub fn insert(&mut self, kind: SpriteKind, entry: Entry, sprite: CachedSprite) {
        self.cache.insert((kind, entry), sprite);
    }

    pub fn get(&self, kind: SpriteKind, entry: &Entry) -> Option<&CachedSprite> {
        self.cache.get(&(kind, *entry))
    }
}

pub fn populate_sprite_cache(
    mut events: MessageReader<AssetEvent<RleAsset>>,
    rle_assets: Res<Assets<RleAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut cache: ResMut<SpriteCache>,
) {
    for event in events.read() {
        let id = match event {
            AssetEvent::Added { id } => *id,
            _ => continue,
        };
        let Some(rle) = rle_assets.get(id) else {
            continue;
        };
        for sprite in &rle.sprites {
            let entry = Entry::new(rle.file_num, sprite.index);
            if cache.cache.contains_key(&(rle.kind, entry)) {
                continue;
            }
            if let Some(image) = rle_to_image(sprite) {
                let handle = images.add(image);
                cache.insert(rle.kind, entry, CachedSprite {
                    handle,
                    x_off: sprite.x_off,
                    y_off: sprite.y_off,
                    width: sprite.width,
                    height: sprite.height,
                });
            }
        }
    }
}

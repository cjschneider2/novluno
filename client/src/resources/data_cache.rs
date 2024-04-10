use std::collections::HashMap;

use bevy::prelude::*;

use crate::asset_loader::rle_asset_loader::SpriteKind;
use crate::asset_loader::rmd_asset_loader::RmdAsset;

#[derive(Resource, Default)]
pub struct DataCache {
    handles: HashMap<(SpriteKind, u32), Handle<RmdAsset>>,
}

impl DataCache {
    pub fn insert(&mut self, kind: SpriteKind, file_num: u32, handle: Handle<RmdAsset>) {
        self.handles.insert((kind, file_num), handle);
    }

    pub fn get(&self, kind: SpriteKind, file_num: u32) -> Option<&Handle<RmdAsset>> {
        self.handles.get(&(kind, file_num))
    }
}

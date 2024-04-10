use std::collections::HashMap;
use bevy::prelude::*;

use crate::asset_loader::map_assset_loader::MapAsset;

#[derive(Default, Resource)]
pub struct MapIndex {
    pub by_number: HashMap<usize, Handle<MapAsset>>
}

impl MapIndex {
    pub fn load_or_get(&mut self, number: usize, asset_server: &AssetServer) -> Handle<MapAsset> {
        if let Some(map) = self.by_number.get(&number) {
            return map.clone();
        }
        let name = format!("data/DATAs/Map/Map{:05}.rmm", number);
        let handle: Handle<MapAsset> = asset_server.load(&name);
        self.by_number.insert(number, handle.clone());
        handle
    }

    pub fn handle(&self, number: usize) -> Option<Handle<MapAsset>> {
        self.by_number.get(&number).cloned()
    }
}
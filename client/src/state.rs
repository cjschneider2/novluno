use crate::asset_loader::list_asset_loader::ListAsset;
use crate::asset_loader::map_assset_loader::MapAsset;
use crate::asset_loader::rmd_asset_loader::RmdAsset;
use crate::config::Config;
use bevy::asset::{AssetServer, Handle};
use bevy::prelude::{DetectChanges, Res, ResMut, Resource};
use bevy_framepace::{FramepaceSettings, Limiter};
use std::collections::HashMap;
use std::path::PathBuf;

type MapAssetHashMap = HashMap<usize, Handle<MapAsset>>;

#[derive(Default)]
pub struct Data {
    pub tile_list: Handle<ListAsset>,
    pub obj_list: Handle<ListAsset>,
    pub maps: MapAssetHashMap,
    /// Character list files, keyed by char_enum (0–9 for PCs, 10 for etc).
    pub chr_lists: HashMap<u32, Handle<ListAsset>>,
    /// Character RMD files, keyed by char_enum.
    pub chr_rmds: HashMap<u32, Handle<RmdAsset>>,
}

#[derive(Resource)]
pub struct State {
    pub current_map: usize,
    pub printed_dbg: bool,
    pub data: Data,
    /// Milliseconds between animation frame advances (default 60 ms ≈ 16.7 fps).
    pub anim_interval_ms: f32,
    /// Render frame rate cap in fps (30–120, default 30).
    pub frame_limit_fps: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_map: 0,
            printed_dbg: false,
            data: Data::default(),
            anim_interval_ms: 60.0,
            frame_limit_fps: 30.0,
        }
    }
}

pub fn load_map_folder_assets(path: &str, asset_server: Res<AssetServer>) -> MapAssetHashMap {
    let mut hashes = HashMap::new();

    let path = PathBuf::new();
    let dir = match path.read_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("Failed to read directory: {}", e)
    };
    dir.for_each(|entry| {
        // parse the number part of the name from each file and use it as a key into the hashmap
        match entry {
            Ok(entry) => {
                let name = entry.file_name().to_str().unwrap().to_string();
                // The file always starts with `Map` and then 5 characters for the number with the
                // extension of `.rmm` for RedMoon Map
                if name.starts_with("Map") && name.len() > 9 {
                    let number = &name[3..7];
                    let number: usize = number.parse().unwrap();
                    let handle = asset_server.load(path.join(name));
                    hashes.insert(number, handle);
                }
            },
            Err(_) => (),
        }
    });

    hashes
}

/// Keeps `FramepaceSettings` in sync with `State::frame_limit_fps`.
/// Runs every frame but only writes the resource when `State` has changed.
pub fn sync_frame_limit(state: Res<State>, mut settings: ResMut<FramepaceSettings>) {
    if state.is_changed() {
        settings.limiter = Limiter::from_framerate(state.frame_limit_fps as f64);
    }
}

pub fn load_assets(mut state: ResMut<State>, asset_server: Res<AssetServer>, _config: Res<Config>) {
    use crate::constants::STARTING_MAP;
    state.data.tile_list = asset_server.load("data/RLEs/tle.lst");
    state.data.obj_list  = asset_server.load("data/RLEs/obj.lst");
    state.data.maps.insert(STARTING_MAP, asset_server.load(format!("data/DATAs/Map/Map{STARTING_MAP:05}.rmm")));
    state.current_map = STARTING_MAP;
    state.printed_dbg = false;
    // Preload character 0 (Philar) assets.
    state.data.chr_lists.insert(0, asset_server.load("data/RLEs/Chr/c00.lst"));
    state.data.chr_rmds.insert(0, asset_server.load("data/DATAs/Chr/chr00000.rmd"));
}

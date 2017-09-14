extern crate core_compat;

use std::path::PathBuf;
use std::path::Path;

use core_compat::DataManager;
use core_compat::MapManager;
use core_compat::SpriteManager;

#[test]
fn test_map_sprite_load_map00001() {
    // setup needed paths
    let data_path = Path::new("../data/DATAs/");
    let map_path = Path::new("../data/DATAs/Map/");
    let sprite_path = Path::new("../data/RLEs/");
    // setup the managers;
    let mut map_manager = MapManager::new(&map_path);
    let mut data_manager = DataManager::new(&data_path);
    let mut sprite_manager = SpriteManager::new(&sprite_path);
    // load the map files
    map_manager.load_map(1).unwrap();
    let map = map_manager.get_map(1).unwrap();
    // check map data
    assert_eq!(map.number, 1);
    assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    // load the data files for each tile
    // for tile in map.
}
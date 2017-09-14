extern crate core_compat;

use std::path::PathBuf;
use std::path::Path;

use core_compat::DataManager;
use core_compat::MapManager;
use core_compat::SpriteManager;
use core_compat::entity::rmd_type::RmdType;

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
    assert_eq!(map.number(), 1);
    assert_eq!((map.size_x() * map.size_y()) as usize, map.tiles().len());
    // load the data files for each tile
    for map_tile in map.tiles().iter() {
        // load references to the data files
        let obj_num = map_tile.object_file_num as usize;
        if obj_num != 0 {
            data_manager.get_data(RmdType::Object, obj_num).unwrap();
        }
        let tile_num = map_tile.tile_file_num as usize;
        if tile_num != 0 {
            data_manager.get_data(RmdType::Tile, tile_num).unwrap();
        }
    }
}

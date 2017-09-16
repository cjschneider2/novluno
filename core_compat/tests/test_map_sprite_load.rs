extern crate core_compat;

use std::path::Path;

use core_compat::DataManager;
use core_compat::MapManager;
use core_compat::SpriteManager;
use core_compat::{ListManager, ListType};
use core_compat::entity::rmd_type::RmdType;
use core_compat::entity::sprite_type::SpriteType;

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
    let list_manager = ListManager::new(sprite_path).unwrap();
    // load the map files
    map_manager.load_map(1).unwrap();
    let map = map_manager.get_map(1).unwrap();
    // check map data
    assert_eq!(map.number(), 1);
    assert_eq!((map.size_x() * map.size_y()) as usize, map.tiles().len());
    // load the data files for each tile
    let obj_list = list_manager.get_list(ListType::Object).unwrap();
    let tle_list = list_manager.get_list(ListType::Tile).unwrap();
    for map_tile in map.tiles().iter() {
        // load references to the data files
        // -- map tile objects
        let obj_entry = map_tile.obj_rmd_entry;
        if obj_entry.file() != 0 {
            let file = obj_entry.file() as usize;
            let index = obj_entry.index() as usize;
            let rmd = data_manager.get_data(RmdType::Object, file).unwrap();
            let entry = rmd.get_entry(index).unwrap();
            println!("Obj entry file: {}", file);
            for img in entry.images() {
                for id in img.get_image_id_list().iter() {
                    println!("\tLooking for obj list item: {}", id);
                    let item = obj_list.get_item(*id as usize).unwrap();
                    // load the RLE's pointed to by the list item
                    let sprite = sprite_manager.get_sprite(item.entry, SpriteType::Object).unwrap();
                }
            }
        }
        // -- map tile sprites
        let tle_entry = map_tile.tle_rmd_entry;
        if tle_entry.file() != 0 {
            let file = tle_entry.file() as usize;
            let index = tle_entry.index() as usize;
            let rmd = data_manager.get_data(RmdType::Tile, file).unwrap();
            let entry = rmd.get_entry(index).unwrap();
            assert!(entry.image_count() > 0);
        }
    }
    assert!(false);
}

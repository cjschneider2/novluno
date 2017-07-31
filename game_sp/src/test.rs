
mod integration {

    use std::path::Path;

    use core_compat::rmd::RmdType;

    use ::map_manager::MapManager;
    use ::data_manager::DataManager;
    use ::sprite_manager::SpriteManager;

    #[test]
    fn map_load() {
        // create the managers
        let map_path = Path::new("../data/DATAs/Map/");
        let data_path = Path::new("../data/DATAs/");
        let sprite_path = Path::new("../data/RLEs/");
        let mut map_manager = MapManager::new(&map_path);
        let mut data_manager = DataManager::new(&data_path);
        let mut sprite_manager = SpriteManager::new(&sprite_path);
        // load the map
        map_manager.load_map(1).unwrap();
        let map = map_manager.get_map(1).unwrap();
        assert_eq!(map.number, 1);
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
        // load the tile data
        let tile = &map.tiles[0];
        let object_num = tile.object_file_num; // points to a `/DATAs/Obj/*.rmd
        let object_idx = tile.object_file_idx; // and it's index into this file
        data_manager.load(RmdType::Object, object_num as usize).unwrap();
        // load the tile rle
    }
}
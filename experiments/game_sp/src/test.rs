mod integration {
    use std::path::Path;


    use core_compat::manager::map_manager::MapManager;
    use core_compat::manager::data_manager::DataManager;
    use core_compat::manager::sprite_manager::SpriteManager;
    use core_compat::entity::rmd_type::RmdType;
    use core_compat::entity::sprite_type::SpriteType;
    use core_compat::entity::entry::Entry;

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
        assert_eq!(map.number(), 1);
        assert_eq!((map.size_x() * map.size_y()) as usize, map.tile_count());
        // load the tile data
        let tile = map.get_tile(0).unwrap();
        let object_num = tile.obj_rmd_entry.file(); // points to a `/DATAs/Obj/*.rmd
        let object_idx = tile.obj_rmd_entry.index(); // and it's index into this file
        data_manager.get_data(RmdType::Object, object_num as usize).unwrap();
        // load the tile rle
    }

    #[test]
    fn load_main_screen() {
        // create the managers
        let sprite_path = Path::new("../data/RLEs/");
        let mut sprite_manager = SpriteManager::new(&sprite_path);
        // load the correct sprite
        let entry = Entry::new(0, 4);
        let interface_t = SpriteType::Interface;
        let sprite = sprite_manager.get_sprite(entry, interface_t).unwrap();
        assert!(sprite.rle_entry == entry);
    }
}

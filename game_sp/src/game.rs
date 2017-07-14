use sprite_manager::SpriteManager;
use map_manager::MapManager;

pub struct Game {
    sprites: SpriteManager,
    maps: MapManager,
}

impl Game {
    pub fn new() -> Game {
        let sqlite_path = ::std::path::Path::new("./rm.sqlite");
        let data_path = ::std::path::Path::new("./data/DATAs/Map/");
        Game {
            sprites: SpriteManager::new(sqlite_path),
            maps: MapManager::new(data_path),
        }
    }
}
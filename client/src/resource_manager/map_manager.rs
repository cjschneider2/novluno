use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use core_compat::entity::map::Map;
// use core_compat::entity::map_tile::MapTile;
use core_compat::parser::rmm::parse_rmm;

use crate::error::Error;

pub struct MapManager {
    data_path: PathBuf,
    maps: HashMap<usize, Rc<Map>>,
}

impl MapManager {

    pub fn new(data_path: &Path) -> MapManager {
        MapManager {
            data_path: data_path.into(),
            maps: HashMap::new(),
        }
    }

    pub fn get_map(&self, number: usize) -> Result<Rc<Map>, Error> {
        let map = match self.maps.get(&number) {
            Some(map) => map.clone(),
            None => return Err(Error::MapLoad),
        };
        Ok(map)
    }

    pub fn load_map(&mut self, number: usize) -> Result<(), Error> {
        // generate correct path for the map
        let map_str = format!("Map{:05}.rmm", number);
        let mut path: PathBuf = self.data_path.clone();
        path.push(&map_str);
        // load data from file
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                println!("Failed to open map file: {:?}", &path);
                return Err(Error::Io(e));
            }
        };
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;
        // parse map and insert into resource_manager
        let map = parse_rmm(&data)?;
        self.maps.insert(number, Rc::new(map));
        println!("Loaded map: {}", &map_str);
        Ok(())
    }

    pub fn get_count(&self) -> usize {
        self.maps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map00001() {
        // load the map files
        let map_data_path = Path::new("../data/DATAs/Map/");
        let mut map_man = MapManager::new(&map_data_path);
        let map_no = 1usize;
        map_man.load_map(map_no).unwrap();
        let map = map_man.maps.get(&1).unwrap();
        assert_eq!(map.number(), 1);
        assert_eq!((map.size_x() * map.size_y()) as usize, map.tile_count());
    }
}

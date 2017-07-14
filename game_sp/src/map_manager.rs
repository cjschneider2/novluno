use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;

use core_compat::rmm::Map;

use error::Error;

pub struct MapManager {
    data_path: PathBuf,
    maps: HashMap<usize, Map>,
}

impl MapManager {

    pub fn new(data_path: &Path) -> MapManager {
        MapManager {
            data_path: data_path.into(),
            maps: HashMap::new(),
        }
    }

    pub fn get_map() -> Result<Map, Error> {
        unimplemented!();
    }

    pub fn load_map(&mut self, number: usize) -> Result<(), Error> {
        // generate correct path for the map
        let map_str = format!("Map{:05}.rmm", number);
        let mut path: PathBuf = self.data_path.clone();
        path.push(map_str);
        // load data from file
        let mut map_file = File::open(&path)?;
        let mut map_data = Vec::<u8>::new();
        map_file.read_to_end(&mut map_data)?;
        // parse map and insert into manager
        let map = Map::load(&map_data)?;
        self.maps.insert(number, map);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map00001() {
        let data_path = Path::new("../data/DATAs/Map/");
        let mut mapper = MapManager::new(&data_path);
        let map_no = 1usize;
        mapper.load_map(map_no).unwrap();
        let map = mapper.maps.get(&1).unwrap();
        assert_eq!(map.number, 1);
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
}
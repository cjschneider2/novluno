
pub struct DataManager {
}

impl DataManager {
    pub fn new() -> DataManager {
        DataManager {}
    }

    pub fn load(&mut self, type: RmdType, number: usize) -> Result<(), Error> {
        // generate correct path for the map
        let map_str = match RmdType {
            format!("{:05}.rmd", number);
        }
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
    fn test_load_Tle_tle_00001() {
        let data_path = Path::new("../data/DATAs/Tle/");
        let mut rmd = DataManager::new(&data_path);
        let rmd_no = 1usize;
        rmd.load_map(rmd_no).unwrap();
        let map = mapper.maps.get(&1).unwrap();
        assert_eq!(map.number, 1);
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
}
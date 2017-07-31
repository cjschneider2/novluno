//! Handles the loading and 

use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use core_compat::rmd::RmdType;
use core_compat::rmd::Rmd;

use error::Error;

pub struct DataManager {
    data_path: PathBuf,
    data: HashMap<(RmdType, usize), Rc<Rmd>>,
}

impl DataManager {
    pub fn new(path: &Path) -> DataManager {
        DataManager {
            data_path: path.into(),
            data: HashMap::new(),
        }
    }

    pub fn load(&mut self, kind: RmdType, number: usize) -> Result<(), Error> {
        // generate correct path for the map
        let mut path: PathBuf = self.data_path.clone();
        let dir_str = match kind {
            RmdType::Tile => { "Tle" },
            RmdType::Object => { "Obj" },
            RmdType::Icon => { "Ico" },
            RmdType::Character => { "Chr" },
            RmdType::Bullet => { "Bul" },
        };
        let map_str = match kind {
            RmdType::Tile => { format!("tle{:05}.rmd", number) },
            RmdType::Object => { format!("obj{:05}.rmd", number) },
            RmdType::Icon => { format!("ico{:05}.rmd", number) },
            RmdType::Character => { format!("chr{:05}.rmd", number) },
            RmdType::Bullet => { format!("bul{:05}.rmd", number) },
        };
        path.push(dir_str);
        path.push(map_str);
        // load data from file
        let mut file = File::open(&path)?;
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;
        // parse map and insert into manager
        let rmd = Rmd::load(kind, &data)?;
        self.data.insert((kind,number), Rc::new(rmd));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_tle_rmd_00001() {
        let data_path = Path::new("../data/DATAs/");
        let mut rmd = DataManager::new(&data_path);
        let rmd_no = 1usize;
        rmd.load(RmdType::Tile, rmd_no).unwrap();
        let data = rmd.data.get(&(RmdType::Tile, 1)).unwrap();
    }
}

use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use entity::rmd_type::RmdType;
use entity::rmd::Rmd;
use parser::rmd::parse_rmd;
use error::Error;

pub struct DataManager {
    data_path: PathBuf,
    tle_map: HashMap<usize, Rc<Rmd>>,
    obj_map: HashMap<usize, Rc<Rmd>>,
    ico_map: HashMap<usize, Rc<Rmd>>,
    chr_map: HashMap<usize, Rc<Rmd>>,
    bul_map: HashMap<usize, Rc<Rmd>>,
}

impl DataManager {
    pub fn new(path: &Path) -> DataManager {
        DataManager {
            data_path: path.into(),
            bul_map: HashMap::new(),
            ico_map: HashMap::new(),
            chr_map: HashMap::new(),
            obj_map: HashMap::new(),
            tle_map: HashMap::new(),
        }
    }

    pub fn get_data(&mut self, kind: RmdType, number: usize) -> Result<Rc<Rmd>, Error> {
        if let Some(rmd) = self.req_data(kind, number) {
            Ok(rmd)
        } else {
            self.load_rmd(kind, number)?;
            if let Some(rmd) = self.req_data(kind, number) {
                Ok(rmd)
            } else {
                Err(Error::DataLoad)
            }
        }
    }

    fn req_data( &self, kind: RmdType, entry: usize) -> Option<Rc<Rmd>> {
        if let Some(entry) = match kind {
            RmdType::Bullet    => { self.bul_map.get(&entry) },
            RmdType::Icon      => { self.ico_map.get(&entry) },
            RmdType::Character => { self.chr_map.get(&entry) },
            RmdType::Object    => { self.obj_map.get(&entry) },
            RmdType::Tile      => { self.tle_map.get(&entry) },
        } {
            Some(entry.clone())
        } else {
            None
        }
    }

    fn load_rmd(&mut self, kind: RmdType, number: usize) -> Result<(), Error> {
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
        path.push(&map_str);
        // load data from file
        // println!("trying to open: {:?}", &path);
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                println!("failed to open RMD file: {:?}", &path);
                return Err(Error::Io(e));
            }
        };
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;
        // parse map and insert into manager
        let rmd = parse_rmd(kind, &data)?;
        match kind {
            RmdType::Tile =>   { self.tle_map.insert(number, Rc::new(rmd)); },
            RmdType::Object => { self.obj_map.insert(number, Rc::new(rmd)); },
            RmdType::Icon =>   { self.ico_map.insert(number, Rc::new(rmd)); },
            RmdType::Character => { self.chr_map.insert(number, Rc::new(rmd)); },
            RmdType::Bullet =>    { self.bul_map.insert(number, Rc::new(rmd)); },
        }
        Ok(())
    }

    pub fn get_count(&self) -> usize {
        self.bul_map.len() +
        self.ico_map.len() +
        self.chr_map.len() +
        self.obj_map.len() +
        self.tle_map.len()
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
        let data = rmd.get_data(RmdType::Tile, 1).unwrap();
    }
}
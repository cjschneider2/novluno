use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::fs::File;
use std::io::Read;

use entity::resource_file::ResourceFile;
use error::Error;
use entity::entry::Entry;
use entity::sprite::Sprite;
use entity::sprite_type::SpriteType::{self, Bullet, Character, Interface, Icon, Tile, Object};
use parser::rle::parse_rle;

pub struct SpriteManager {
    db_path: PathBuf,
    bul_map: HashMap<Entry, Rc<>>,
    ico_map: HashMap<Entry, Rc<>>,
    chr_map: HashMap<Entry, Rc<>>,
    obj_map: HashMap<Entry, Rc<>>,
    tle_map: HashMap<Entry, Rc<>>,
    int_map: HashMap<Entry, Rc<>>,
}

impl SpriteManager {
    pub fn new(db_path: &Path) -> SpriteManager {
        SpriteManager {
            db_path: db_path.into(),
            bul_map: HashMap::new(),
            ico_map: HashMap::new(),
            chr_map: HashMap::new(),
            obj_map: HashMap::new(),
            tle_map: HashMap::new(),
            int_map: HashMap::new(),
        }
    }

    pub fn get_sprite(
        &mut self,
        req_entry: Entry,
        sprite_type: SpriteType
    ) -> Result<Rc<Sprite>, Error> {
        if let Some(sprite) = self.req_sprite(&req_entry, sprite_type) {
            Ok(sprite)
        } else {
            self.load_sprite(req_entry.file(), sprite_type)?;
            if let Some(sprite) = self.req_sprite(&req_entry, sprite_type) {
                Ok(sprite)
            } else {
                Err(Error::SpriteLoad)
            }
        }
    }

    fn req_sprite(
        &self,
        entry: &Entry,
        sprite_type: SpriteType
    ) -> Option<Rc<Sprite>> {
        if let Some(entry) = match sprite_type {
            Bullet    => { self.bul_map.get(entry) },
            Icon      => { self.ico_map.get(entry) },
            Character => { self.chr_map.get(entry) },
            Object    => { self.obj_map.get(entry) },
            Tile      => { self.tle_map.get(entry) },
            Interface => { self.int_map.get(entry) },
        } {
            Some(entry.clone())
        } else {
            None
        }
    }

    fn load_sprite(
        &mut self,
        number: u32,
        sprite_type: SpriteType
    ) -> Result<(), Error> {
        // generate correct path for the sprite
        let folder_str = match sprite_type {
            Bullet    => {"Bul"},
            Icon      => {"Ico"},
            Character => {"Chr"},
            Object    => {"Obj"},
            Tile      => {"Tle"},
            Interface => {"Int"},
        };
        let file_str = match sprite_type {
            Bullet    => format!("bul{:05}.rle", number),
            Icon      => format!("ico{:05}.rle", number),
            Character => format!("chr{:05}.rle", number),
            Object    => format!("obj{:05}.rle", number),
            Tile      => format!("tle{:05}.rle", number),
            Interface => format!("int{:05}.rle", number),
        };
        let mut path: PathBuf = self.db_path.clone();
        path.push(folder_str);
        path.push(file_str);
        // load data
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to load sprite file: {:?}", &path);
                return Err(Error::Io(e))
            }
        };
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;
        // parse rle file and insert into resource_manager
        let resource_file = parse_rle(number, &data)?;
        for resource in resource_file.resources {
            let entry = Entry::new(number, resource.index() );
            let sprite = Sprite {
                class: sprite_type,
                rle_entry: entry,
                x_dim: resource.width as usize,
                y_dim: resource.height as usize,
                x_off: resource.offset_x as usize,
                y_off: resource.offset_y as usize,
                image: resource.image,
            };
            match sprite_type {
                Bullet    => { self.bul_map.insert(entry, Rc::new(sprite)); },
                Icon      => { self.ico_map.insert(entry, Rc::new(sprite)); },
                Character => { self.chr_map.insert(entry, Rc::new(sprite)); },
                Object    => { self.obj_map.insert(entry, Rc::new(sprite)); },
                Tile      => { self.tle_map.insert(entry, Rc::new(sprite)); },
                Interface => { self.int_map.insert(entry, Rc::new(sprite)); },
            }
        }
        Ok(())
    }

    pub fn get_count(&self) -> usize {
          self.bul_map.len()
        + self.ico_map.len()
        + self.chr_map.len()
        + self.obj_map.len()
        + self.tle_map.len()
        + self.int_map.len()
    }
}

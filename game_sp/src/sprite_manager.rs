use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::fs::File;
use std::io::Read;

use core_compat::rle::ResourceFile;

use error::Error;
use entry::Entry;
use sprite::Sprite;
use sprite_type::SpriteType::{
    self, Bullet, Character, Interface, Icon, Tile, Object};

pub struct SpriteManager {
    db_path: PathBuf,
    // FIXME: doing seperate maps really stinks...
    map_bul: HashMap<Entry, Rc<Sprite>>,
    map_ico: HashMap<Entry, Rc<Sprite>>,
    map_chr: HashMap<Entry, Rc<Sprite>>,
    map_obj: HashMap<Entry, Rc<Sprite>>,
    map_tle: HashMap<Entry, Rc<Sprite>>,
    map_int: HashMap<Entry, Rc<Sprite>>,
}

impl SpriteManager {
    pub fn new(db_path: &Path) -> SpriteManager {
        SpriteManager {
            db_path: db_path.into(),
            map_bul: HashMap::new(),
            map_ico: HashMap::new(),
            map_chr: HashMap::new(),
            map_obj: HashMap::new(),
            map_tle: HashMap::new(),
            map_int: HashMap::new(),
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
            self.load_sprite(req_entry.file, sprite_type)?;
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
            Bullet    => { self.map_bul.get(entry) },
            Icon      => { self.map_ico.get(entry) },
            Character => { self.map_chr.get(entry) },
            Object    => { self.map_obj.get(entry) },
            Tile      => { self.map_tle.get(entry) },
            Interface => { self.map_int.get(entry) },
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
        let file_str = format!("int{:05}.rle", number);
        let folder_str = match sprite_type {
            Bullet    => {"Bul"},
            Icon      => {"Ico"},
            Character => {"Chr"},
            Object    => {"Obj"},
            Tile      => {"Tle"},
            Interface => {"Int"},
        };
        let mut path: PathBuf = self.db_path.clone();
        path.push(folder_str);
        path.push(file_str);
        // load data
        let mut file = File::open(&path)?;
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data)?;
        // parse rle file and insert into manager
        let resource_file = ResourceFile::load(number, &data)?;
        for resource in resource_file.resources {
            let entry = Entry { file: number, index: resource.index };
            let sprite = Sprite {
                class: sprite_type,
                entry: entry,
                x_dim: resource.width as usize,
                y_dim: resource.height as usize,
                x_off: resource.offset_x as usize,
                y_off: resource.offset_y as usize,
                image: resource.image,
            };
            match sprite_type {
                Bullet    => { self.map_bul.insert(entry, Rc::new(sprite)); },
                Icon      => { self.map_ico.insert(entry, Rc::new(sprite)); },
                Character => { self.map_chr.insert(entry, Rc::new(sprite)); },
                Object    => { self.map_obj.insert(entry, Rc::new(sprite)); },
                Tile      => { self.map_tle.insert(entry, Rc::new(sprite)); },
                Interface => { self.map_int.insert(entry, Rc::new(sprite)); },
            }
        }
        Ok(())
    }

}

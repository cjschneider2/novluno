use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::fs::File;
use std::io::Read;

use sdl2;

// use core_compat::entity::resource_file::ResourceFile;
use core_compat::entity::entry::Entry;
use core_compat::entity::sprite::Sprite;
use core_compat::entity::sprite_type::SpriteType::{self, Bullet, Character, Interface, Icon, Tile, Object};
use core_compat::parser::rle::parse_rle;

use error::Error;
use sdl::Sdl;

pub struct SpriteEntry {
    pub sprite: Sprite,
    pub texture: sdl2::render::Texture
}

pub struct SpriteManager {
    db_path: PathBuf,
    bul_map: HashMap<Entry, Rc<SpriteEntry>>,
    ico_map: HashMap<Entry, Rc<SpriteEntry>>,
    chr_map: HashMap<Entry, Rc<SpriteEntry>>,
    obj_map: HashMap<Entry, Rc<SpriteEntry>>,
    tle_map: HashMap<Entry, Rc<SpriteEntry>>,
    int_map: HashMap<Entry, Rc<SpriteEntry>>,
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

    pub fn get_sprite_entry(
        &mut self,
        req_entry: &Entry,
        sprite_type: SpriteType,
        sdl: &mut Sdl
    ) -> Result<&SpriteEntry, Error> {

        // TODO: Work around borrowing rules to only request the sprite once...
        {
            let mut need_load = false;

            if self.req_sprite(req_entry, sprite_type).is_none() {
                need_load = true;
            }

            if need_load {
                self.load_sprite(req_entry.file(), sprite_type, sdl)?;
            }
        }

        if let Some(entry) = self.req_sprite(req_entry, sprite_type) {
            Ok(entry)
        } else {
            Err(Error::SpriteLoad)
        }
    }

    fn req_sprite(
        &self,
        entry: &Entry,
        sprite_type: SpriteType
    ) -> Option<&SpriteEntry> {
        if let Some(entry) = match sprite_type {
            Bullet    => { self.bul_map.get(entry) },
            Icon      => { self.ico_map.get(entry) },
            Character => { self.chr_map.get(entry) },
            Object    => { self.obj_map.get(entry) },
            Tile      => { self.tle_map.get(entry) },
            Interface => { self.int_map.get(entry) },
        } {
            Some(entry)
        } else {
            None
        }
    }

    fn load_sprite(
        &mut self,
        number: u32,
        sprite_type: SpriteType,
        sdl: &mut Sdl
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
                x_dim: resource.width,
                y_dim: resource.height,
                x_off: resource.offset_x,
                y_off: resource.offset_y,
                image_raw: resource.image_raw,
            };

            let mut texture = sdl.texture_creator.create_texture(
                Some(sdl2::pixels::PixelFormatEnum::ABGR8888),
                sdl2::render::TextureAccess::Static,
                resource.width as u32,
                resource.height as u32)?;
            texture.set_blend_mode(sdl2::render::BlendMode::Blend);
            let pitch = resource.width as usize * 4;
            texture.update(None, &sprite.image_raw, pitch).unwrap();

            let sprite_entry = Rc::new(SpriteEntry { sprite, texture });

            match sprite_type {
                Bullet    => { self.bul_map.insert(entry, sprite_entry); },
                Icon      => { self.ico_map.insert(entry, sprite_entry); },
                Character => { self.chr_map.insert(entry, sprite_entry); },
                Object    => { self.obj_map.insert(entry, sprite_entry); },
                Tile      => { self.tle_map.insert(entry, sprite_entry); },
                Interface => { self.int_map.insert(entry, sprite_entry); },
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

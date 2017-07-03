use std::collections::HashMap;
use std::path::{Path, PathBuf};

use error::Error;
use entry::Entry;
use sprite::Sprite;

pub struct SpriteManager {
    db_path: PathBuf,
    map: HashMap<Entry, Sprite>,
}

impl SpriteManager {
    pub fn new(db_path: &Path) -> SpriteManager {
        SpriteManager {
            db_path: db_path.into(),
            map: HashMap::new(),
        }
    }

    pub fn get_sprite(&mut self, entry: Entry) -> Result<&Sprite, Error>{
        // First check to see if we've already loaded the sprite file
        if let Some(item) = self.map.get(&entry) {
            Ok(item)
        } else {
            Err(Error::SpriteLoad)
        }
    }
}

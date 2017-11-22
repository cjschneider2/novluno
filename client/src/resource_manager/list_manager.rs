use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::rc::Rc;

use error::Error;
use entity::list::List;
use entity::list_item::ListItem;
use parser::lst::parse_lst;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum ListType {
    Bullet,
    Icon,
    Interface,
    Sound,
    Tile,
    Object,
    Chr0,
    Chr1,
    Chr2,
    Chr3,
    Chr4,
    Chr5,
    Chr6,
    Chr7,
    Chr8,
    Chr9,
    Etc,
}

static LIST_PATHS: [(&'static ListType, &'static str); 5] = [
    (&ListType::Bullet, "bul.lst"),
    (&ListType::Icon, "ico.lst"),
    (&ListType::Interface, "int.lst"),
    (&ListType::Tile, "tle.lst"),
    (&ListType::Object, "obj.lst"),
];

pub struct ListManager {
    rle_path: PathBuf,
    list_map: HashMap<ListType, Rc<List>>,
}

impl ListManager {
    pub fn new(rle_path: &Path) -> Result<ListManager, Error> {
        let mut lm = ListManager {
            rle_path: rle_path.into(),
            list_map: HashMap::new(),
        };
        for entry in LIST_PATHS.iter() {
            let (kind, path) = *entry;
            // create path
            let mut next_path = lm.rle_path.clone();
            next_path.push(path);
            // open and read file
            println!("Loading list file: {:?}", next_path);
            let mut file = File::open(&next_path)?;
            let mut data = Vec::<u8>::new();
            file.read_to_end(&mut data)?;
            // parse data
            let list_file = parse_lst(&data, false)?;
            // save it in map
            lm.list_map.insert(*kind, Rc::new(list_file));
        }
        Ok(lm)
    }

    pub fn get_list(&self, kind: ListType) -> Option<Rc<List>> {
        if let Some(entry) = self.list_map.get(&kind) {
            Some(entry.clone())
        } else {
            None
        }
    }

    pub fn get_count(&self) -> usize {
        self.list_map.len()
    }
}
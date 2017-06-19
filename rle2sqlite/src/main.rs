extern crate rm_core;

use std::collections::HashSet;
use std::path::Path;

use rm_core::read_rle_dir::read_rle_dir;

static FOLDER_ENTRIES: [(&'static str, &'static str, &'static str); 5] = [
    ("Bullets", "../data/RLEs/Bul", "../data/RLEs/bul.lst"),
    ("Icons", "../data/RLEs/Ico", "../data/RLEs/ico.lst"),
    ("Objects", "../data/RLEs/Obj", "../data/RLEs/obj.lst"),
    ("Tiles", "../data/RLEs/Tle", "../data/RLEs/tle.lst"),
    ("Interface", "../data/RLEs/Int", "../data/RLEs/int.lst"),
    // The sounds one is the only one which is a little different...
    // ("Sounds", "../data/RLEs/Snd", "../data/RLEs/snd.lst"),
];

fn main() {

    let mut map = HashSet::<u32>::new();
    for &(_type, folder, list) in FOLDER_ENTRIES.iter() {
        println!("Parsing: {:?}", _type);
        let data_folder = Path::new(folder);
        let data_list = Path::new(list);
        let entries = read_rle_dir(&data_list, &data_folder).unwrap();
        for entry in entries {
            if let Some(id) = entry.id {
                let success = map.insert(id);
                if !success {
                    // testing for ID doubles
                    // println!("double id: {:?}", &id);
                }
            } else {
                // testing missing ID
                // println!("missing id");
            }
        }
    }
    println!("number of entries: {:?}", map.len());
}


use std::io::prelude::*;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::fs::read_dir;
use std::io::BufReader;

use error::Error;
use rle;
use lst;

pub fn read_rle_dir(rle_list: &Path, rle_folder: &Path) -> Result<Vec<rle::Resource>, Error> {
    // load and parse the list file
    let mut rle_list_file = File::open(rle_list)?;
    let mut list_bytes = Vec::<u8>::new();
    rle_list_file.read_to_end(&mut list_bytes)?;
    let list = lst::List::load(&list_bytes,false)?;

    // load and parse the rle files
    let rle_file_paths = read_dir(rle_folder)?;
    let mut resource_files = Vec::<rle::ResourceFile>::new();
    for entry in rle_file_paths {
        let entry = entry?;
        let mut file = File::open(entry.path())?;
        let mut bytes = Vec::<u8>::new();
        file.read_to_end(&mut bytes)?;
        let mut res_file = rle::ResourceFile::load(&mut bytes)?;
        if let Some(stem) = entry.path().file_stem() {
            if let Some(stem) = stem.to_str() {
                let num: String = stem.matches(char::is_numeric).collect();
                let num: u32 = num.parse().unwrap_or(0xFFFF);
                res_file.file_number = num;
            }
        }
        if let Some(name) = entry.path().file_name() {
            if let Some(name) = name.to_str() {
                res_file.name = name.to_owned();
            }
        }
        resource_files.push(res_file);
    }

    // sort out the list entries with their respective resource
    let mut entities = Vec::new();
    for rle_file in resource_files {
        for mut rle in rle_file.resources {
            for entry in &list.items {
                if entry.file_number == rle_file.file_number && entry.index == rle.index {
                    rle.name = Some(entry.name.clone());
                    rle.id = Some(entry.id);
                    rle.lst_rle_index = Some(entry.index);
                    rle.file_number = Some(rle_file.file_number);
                }
            }
            entities.push(rle);
        }
    }

    Ok(entities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_bul_dir() {
        let data_folder = Path::new("../data/RLEs/Bul");
        let data_list = Path::new("../data/RLEs/bul.lst");
        let entries = read_rle_dir(&data_list, &data_folder).unwrap();
        let mut map = HashSet::<u32>::new();
        for entry in entries {
            if let Some(id) = entry.id {
                let success = map.insert(id);
                if !success {
                    // testing for ID doubles
                    assert!(false);
                }
            } else {
                // testing missing ID
                assert!(false);
            }
        }
    }
}

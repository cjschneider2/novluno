//! This program reads the RLE sprite sheets and list files which contain the
//! id's for the sprite type, and converts them into an sqlite database. While
//! an sqlite database maybe isn't the most efficient, it's at least somewhat
//! portable and quick to iterate with. Let alone compressing and transferring.
//!
//! NOTES:
//!  - So it seems that the ID value in the list file isn't global to the entire
//!    game, and instead only global to the list file itself. So at this point
//!    I'm thinking that assigning a global ID might be a good idea, though
//!    this ID would just be for referencing the objects which we pull, and not
//!    between objects because they could change depending on the input data.

extern crate core_compat;
extern crate rusqlite as sql;

use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use core_compat::read_rle_dir::read_rle_dir;
use core_compat::lst::{List, ListItem};
use core_compat::error::Error;

use sql::Connection;

// This is the list of data folder's and list files for them
static FOLDER_ENTRIES: [(&'static str, &'static str, &'static str); 1] = [
    ("Bullets", "../data/RLEs/Bul", "../data/RLEs/bul.lst"),
    // ("Icons", "../data/RLEs/Ico", "../data/RLEs/ico.lst"),
    // ("Objects", "../data/RLEs/Obj", "../data/RLEs/obj.lst"),
    // ("Tiles", "../data/RLEs/Tle", "../data/RLEs/tle.lst"),
    // ("Interface", "../data/RLEs/Int", "../data/RLEs/int.lst"),
    // The sounds one is the only one which is a little different...
    // ("Sounds", "../data/RLEs/Snd", "../data/RLEs/snd.lst"),
];

fn main() {

    // create sqlite database
    // let connection = Connection::open_in_memory().unwrap();
    let mut connection = Connection::open(Path::new("./rm.sqlite")).unwrap();

    connection.execute("DROP TABLE list", &[]).unwrap();

    connection.execute(
        "CREATE TABLE list (
            gid      INTEGER PRIMARY KEY,
            list_id  INTEGER,
            file_num INTEGER,
            file_idx INTEGER,
            type     TEXT NOT NULL,
            name     TEXT NOT NULL
        )", &[]).unwrap();

    // parse the list file and insert them into the database
    for &(_type, folder, list) in FOLDER_ENTRIES.iter() {
        // load the data from the file
        let data_list = Path::new(list);
        let list = load_list_data(&data_list).unwrap();
        println!("list.items.len() == {:?}", list.items.len());
        let tx = connection.transaction().unwrap();
        for item in list.items {
            // println!("inserting: {:?}", item);
            // insert the data into the database
            tx.execute(
                "INSERT INTO list (type, name, list_id, file_num, file_idx)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                 &[&_type, &item.name, &item.id, &item.file_number, &item.index]
            ).unwrap();
        }
        tx.commit().unwrap();
    }

    // // check the # of entries in the database
    // let mut stmt = connection.prepare("SELECT list_id, name FROM list").unwrap();
    // let lst_itr = stmt.query_map(&[], |row| {
    //     println!("retrieved row");
    //     let id: u32 = row.get(1);
    //     let name: String = row.get(2);
    //     (id, name)
    // }).unwrap();
    // let lst_vec = lst_itr.filter_map(|x| x.ok()).collect::<Vec<(u32, String)>>();
    // println!("lst_vec.len(): {:?}", lst_vec.len());
}

fn load_list_data(list_path: &Path) -> Result<List, Error> {
    let mut list_file = File::open(list_path)?;
    let mut list_bytes = Vec::<u8>::new();
    list_file.read_to_end(&mut list_bytes)?;
    List::load(&list_bytes, false)
}

#[allow(dead_code)]
fn parse_entries() {
    // parse entries
    for &(_type, folder, list) in FOLDER_ENTRIES.iter() {
        println!("Parsing: {:?}", _type);
        let mut map = HashSet::<u32>::new();

        let data_folder = Path::new(folder);
        let data_list = Path::new(list);

        let entries = read_rle_dir(&data_list, &data_folder).unwrap();
        let mut missing_ids = 0;

        for entry in entries {
            if let Some(id) = entry.id {
                let success = map.insert(id);
                if !success {
                    // testing for ID doubles
                    println!("double id: {:?}", &id);
                }
            } else {
                // testing missing ID
                missing_ids += 1;
            }
        }
        println!("\tentries      : {:?}", map.len());
        println!("\tmissing id's : {:?}", missing_ids);
    }
}
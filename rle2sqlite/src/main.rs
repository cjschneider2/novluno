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
//!  - The best way it seems to match the data from the `rle` and `list` tables
//!    is to use the file number and file index

extern crate core_compat;
extern crate rusqlite as sql;

use std::path::Path;
use std::fs::File;
use std::fs::read_dir;
use std::io::Read;

use core_compat::entity::resource_file::ResourceFile;
use core_compat::entity::resource::Resource;
use core_compat::entity::list::List;
use core_compat::error::Error;
use core_compat::parser::rle::parse_rle;
use core_compat::parser::lst::parse_lst;

use sql::Connection;

// This is the list of data folder's and list files for them
static FOLDER_ENTRIES: [(&'static str, &'static str, &'static str); 1] = [
    // ("Bullets", "../data/RLEs/Bul", "../data/RLEs/bul.lst"),
    // ("Icons", "../data/RLEs/Ico", "../data/RLEs/ico.lst"),
    // ("Objects", "../data/RLEs/Obj", "../data/RLEs/obj.lst"),
    // ("Tiles", "../data/RLEs/Tle", "../data/RLEs/tle.lst"),
    ("Interface", "../data/RLEs/Int", "../data/RLEs/int.lst"),
    // The sounds one is the only one which is a little different...
    // ("Sounds", "../data/RLEs/Snd", "../data/RLEs/snd.lst"),
];

fn main() {

    // create sqlite database
    // let connection = Connection::open_in_memory().unwrap();
    let mut connection = Connection::open(Path::new("./rm.sqlite")).unwrap();

    let _ = connection.execute("DROP TABLE list", &[]);
    let _ = connection.execute("DROP TABLE rle", &[]);

    connection.execute(
        "CREATE TABLE list (
            gid      INTEGER PRIMARY KEY,
            type     TEXT NOT NULL,
            file_num INTEGER,
            file_idx INTEGER,
            name     TEXT NOT NULL,
            list_id  INTEGER
        )", &[]).unwrap();

    connection.execute(
        "CREATE TABLE rle (
            gid      INTEGER PRIMARY KEY,
            type     TEXT NOT NULL,
            file_num INTEGER,
            file_idx INTEGER,
            length   INTEGER,
            offset_x INTEGER,
            offset_y INTEGER,
            width    INTEGER,
            height   INTEGER,
            image    BLOB
        )", &[]).unwrap();

    // parse the list file and insert them into the database
    for &(_type, folder, list) in FOLDER_ENTRIES.iter() {

        println!("file: {:?}", _type);

        // load the data from the list file
        let list_path = Path::new(list);
        let list = load_list_data(&list_path).unwrap();
        println!("list.items.len() == {:?}", list.items.len());

        // Commit all of the list objects in one transaction
        {
            let tx = connection.transaction().unwrap();
            for item in list.items {
                // insert the data into the database
                tx.execute(
                    "INSERT INTO list (
                        type, name, list_id, file_num, file_idx)
                    VALUES (?1, ?2, ?3, ?4, ?5)",
                    &[&_type, &item.name, &item.id,
                      &item.file_number, &item.index]
                ).unwrap();
            }
            tx.commit().unwrap();
        }

        // load the actual sprites into the database
        let rle_paths = read_dir(folder).unwrap();
        let mut resources = Vec::<Resource>::new();

        for entry in rle_paths {

            let entry = entry.unwrap();
            let path = entry.path();

            let res_file: ResourceFile = load_rle_data(&path).unwrap();

            for resource in res_file.resources {
                resources.push(resource);
            }

        }

        // Commit all of the sprite objects in one transaction
        {
            let tx = connection.transaction().unwrap();
            for ref rle in &resources {

                // TODO: hack the Vec<Pixel> into a Vec<U8>
                let mut img = Vec::<u8>::new();
                for ref pixel in &rle.image {
                    img.push(pixel.r);
                    img.push(pixel.g);
                    img.push(pixel.b);
                    img.push(pixel.a);
                }

                // insert the data into the database
                tx.execute(
                    "INSERT INTO rle (
                        type,   file_num, file_idx,
                        length, offset_x, offset_y,
                        width,  height,   image)
                    VALUES (?1, ?2, ?3,
                            ?4, ?5, ?6,
                            ?7, ?8, ?9)",
                    &[&_type,   &rle.file_num, &rle.index,
                    &rle.len,   &rle.offset_x, &rle.offset_y,
                    &rle.width, &rle.height,   &img]
                ).unwrap();
            }
            tx.commit().unwrap();

        }
        println!("resources.len() == {:?}", &resources.len());
    }

    // check the # of entries in the database
    let mut stmt = connection.prepare("SELECT list_id, name FROM list").unwrap();
    let lst_itr = stmt.query_map(&[], |row| {
        let id: u32 = row.get(0);
        let name: String = row.get(1);
        (id, name)
    }).unwrap();
    let lst_vec = lst_itr.filter_map(|x| x.ok()).collect::<Vec<_>>();
    println!("lst_vec.len(): {:?}", lst_vec.len());
}

fn load_list_data(path: &Path) -> Result<List, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;
    parse_lst(&bytes, false)
}

fn load_rle_data(path: &Path) -> Result<ResourceFile, Error> {

    // open and read the file
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;

    // parse the file number
    let mut file_num = 0xFFFF;
    if let Some(stem) = path.file_stem() {
        if let Some(stem) = stem.to_str() {
            let num: String = stem.matches(char::is_numeric).collect();
            file_num = num.parse().unwrap_or(0xFFFF);
        }
    }

    // parse && append results
    parse_rle(file_num, &mut bytes)
}

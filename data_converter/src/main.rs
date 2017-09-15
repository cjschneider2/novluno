#![allow(dead_code, unused_variables)]

extern crate core_compat;
extern crate png;
extern crate xml_writer;

use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::fs::read_dir;
use std::io::Read;
// use std::io::Write;
use std::io::BufWriter;

use png::HasParameters;

use core_compat::entity::resource_file::ResourceFile;
use core_compat::entity::resource::Resource;
use core_compat::entity::rmd::Rmd;
use core_compat::entity::rmd_type::RmdType;
use core_compat::entity::map::Map;
use core_compat::entity::list::List;
use core_compat::error::Error;
use core_compat::parser::rle::parse_rle;
use core_compat::parser::rmd::parse_rmd;
use core_compat::parser::rmm::parse_rmm;
use core_compat::parser::lst::parse_lst;

static OUTPUT_PATH: &'static str = "../temp/";

// This is the list of data folder's and list files for them
static RLE_ENTRIES: [(&'static str, &'static str, &'static str,
                      &'static str, bool); 16] = [
    ("bullets", "bul", "../data/RLEs/Bul", "../data/RLEs/bul.lst", false),
    ("icons", "ico", "../data/RLEs/Ico", "../data/RLEs/ico.lst", false),
    ("objects", "obj", "../data/RLEs/Obj", "../data/RLEs/obj.lst", true),
    ("tiles", "tle", "../data/RLEs/Tle", "../data/RLEs/tle.lst", false),
    ("interface", "int", "../data/RLEs/Int", "../data/RLEs/int.lst", false),
    ("philar", "ch0", "../data/RLEs/Chr/C00", "../data/RLEs/Chr/c00.lst", false),
    ("azlar", "ch1", "../data/RLEs/Chr/C01", "../data/RLEs/Chr/c01.lst", false),
    ("sadad", "ch2", "../data/RLEs/Chr/C02", "../data/RLEs/Chr/c02.lst", false),
    ("destino", "ch3", "../data/RLEs/Chr/C03", "../data/RLEs/Chr/c03.lst", false),
    ("jarexx", "ch4", "../data/RLEs/Chr/C04", "../data/RLEs/Chr/c04.lst", false),
    ("canon", "ch5", "../data/RLEs/Chr/C05", "../data/RLEs/Chr/c05.lst", false),
    ("kitara", "ch6", "../data/RLEs/Chr/C06", "../data/RLEs/Chr/c06.lst", false),
    ("lunarena", "ch7", "../data/RLEs/Chr/C07", "../data/RLEs/Chr/c07.lst", false),
    ("lavita", "ch8", "../data/RLEs/Chr/C08", "../data/RLEs/Chr/c08.lst", false),
    ("ch_9_gm", "ch9", "../data/RLEs/Chr/C09", "../data/RLEs/Chr/c09.lst", false),
    ("extra_chr", "etc", "../data/RLEs/Chr/Etc", "../data/RLEs/Chr/etc.lst", false),
    // The sounds one is the only one which is a little different...
    // ("Sounds", "snd", "../data/RLEs/Snd", "../data/RLEs/snd.lst"),
];

static RMM_ENTRY: (&'static str, &'static str) =
    ("maps", "../data/DATAs/Map");

static RMD_ENTRIES: [(&'static str, &'static str, &'static str, RmdType); 5] = [
    ("bullet", "bul", "../data/DATAs/Bul", RmdType::Bullet),
    ("char", "chr", "../data/DATAs/Chr", RmdType::Character),
    ("icon", "ico", "../data/DATAs/Ico", RmdType::Icon),
    ("object", "obj", "../data/DATAs/Obj", RmdType::Object),
    ("tile", "tle", "../data/DATAs/Tle", RmdType::Tile),
];

fn main() {
    // create directory - print errors...
    let root_out_dir = Path::new(OUTPUT_PATH);
    println!("Creating directory: {:?}", root_out_dir);
    match std::fs::create_dir(root_out_dir) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    // parse the list file and insert them into the database
    // convert_rle_data();

    // convert the maps ...
    // convert_rmm_data();

    // ... and rmd files
    convert_rmd_data();
}

fn convert_rmd_data() {
    // create the output directory if it doesn't exist yet
    let mut data_out_dir = PathBuf::new();
    data_out_dir.push(OUTPUT_PATH);
    data_out_dir.push("data");
    println!("Creating directory: {:?}", data_out_dir);
    match std::fs::create_dir(data_out_dir) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    // read every folder
    for &(kind, short, path, rmd_type) in RMD_ENTRIES.iter() {
        let data_paths = read_dir(path).unwrap();

        // read every file
        for entry in data_paths {
            let entry = entry.unwrap();
            let path = entry.path();

            let dat_file: Rmd = load_rmd_data(&path, rmd_type).unwrap();
        }
    }
}

fn convert_rmm_data() {
    // create the output directory if it doesn't exist yet
    let mut map_out_dir = PathBuf::new();
    map_out_dir.push(OUTPUT_PATH);
    map_out_dir.push("map");
    println!("Creating directory: {:?}", map_out_dir);
    match std::fs::create_dir(map_out_dir) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    // book-keeping of map data paths
    let (kind, path) = RMM_ENTRY;
    let map_path = Path::new(path);
    let map_file_paths = read_dir(map_path).unwrap();

    // parse the map files in the map directory
    let mut map_list: Vec<Map> = Vec::new();
    for entry in map_file_paths {
        let entry = entry.unwrap();
        let path = entry.path();
        let map: Map = match load_rmm_data(&path) {
            Ok(map) => map,
            Err(e) => {
                println!("{:?}", e);
                println!("{:?}", path);
                continue
            }
        };
        map_list.push(map);
    }
    println!("parsed {} map entries.", map_list.len());

    // export the files as xml data in the output directory
    for map in map_list {
        let map_out_file_name = format!("{}_{:03}.xml", kind, map.number());
        let mut path_buf = PathBuf::new();
        path_buf.push(OUTPUT_PATH);
        path_buf.push("map");
        path_buf.push(map_out_file_name);

        let file = File::create(&path_buf).unwrap();
        let writer = BufWriter::new(file);

        let mut xml = xml_writer::XmlWriter::new(writer);
        xml.begin_elem("map").unwrap();
        // map number
        xml.begin_elem("number").unwrap();
        xml.text(&format!("{}", map.number())).unwrap();
        xml.end_elem().unwrap();
        // size_x
        xml.begin_elem("size_x").unwrap();
        xml.text(&format!("{}", map.size_x())).unwrap();
        xml.end_elem().unwrap();
        // size_y
        xml.begin_elem("size_y").unwrap();
        xml.text(&format!("{}", map.size_y())).unwrap();
        xml.end_elem().unwrap();
        // events
        // TODO: The exported events seem a little wonky...
        /*
        for event in map.events {
            xml.begin_elem("event").unwrap();
            xml.attr("number", &format!("{}", event.number)).unwrap();
            xml.attr("left", &format!("{}", event.left)).unwrap();
            xml.attr("top", &format!("{}", event.top)).unwrap();
            xml.attr("right", &format!("{}", event.right)).unwrap();
            xml.attr("bottom", &format!("{}", event.bottom)).unwrap();
            xml.end_elem().unwrap();
        }
        */
        // tiles
        let mut x = 0;
        let mut y = 0;
        let max_x = map.size_x();
        let max_y = map.size_y();

        for tile in map.tiles() {
            // <tile>
            xml.begin_elem("tile").unwrap();
            // <x>
            xml.begin_elem("x").unwrap();
            xml.text(&format!("{}", &format!("{}", x))).unwrap();
            xml.end_elem().unwrap();
            // <y>
            xml.begin_elem("y").unwrap();
            xml.text(&format!("{}", &format!("{}", y))).unwrap();
            xml.end_elem().unwrap();
            // <object_ref> rm data reference
            xml.begin_elem("object_ref").unwrap();
            xml.attr("file", &format!("{}", tile.obj_rmd_entry.file())).unwrap();
            xml.attr("index", &format!("{}", tile.obj_rmd_entry.index())).unwrap();
            xml.end_elem().unwrap();
            // <tile_ref> rm data reference
            xml.begin_elem("tile_ref").unwrap();
            xml.attr("file", &format!("{}", tile.tle_rmd_entry.file())).unwrap();
            xml.attr("index", &format!("{}", tile.tle_rmd_entry.index())).unwrap();
            xml.end_elem().unwrap();
            // <warp>
            xml.begin_elem("warp").unwrap();
            xml.text(&format!("{}", tile.warp)).unwrap();
            xml.end_elem().unwrap();
            // <collision>
            xml.begin_elem("collision").unwrap();
            xml.text(&format!("{}", tile.collision)).unwrap();
            xml.end_elem().unwrap();
            // </tile>
            xml.end_elem().unwrap();

            // handle coordinate increments
            x += 1;
            if x >= max_x {
                y += 1;
                x = 0;
            }
        }

        if y != max_y {
            println!("Map dimension mis-match: y:{}, max_y: {}", y, max_y);
        }

        xml.close().unwrap();
        xml.flush().unwrap();
    }
}

fn convert_rle_data() {
    for &(kind, short_kind, folder, list, use_v2) in RLE_ENTRIES.iter() {
        println!("file: {:?}", &kind);

        // load the data from the list file
        let list_path = Path::new(list);
        let list = load_list_data(&list_path, use_v2).unwrap();

        println!("list.items.len() == {:?}", list.items.len());

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
        let mut combi_entries: Vec<RleCombiEntry> = Vec::new();
        let mut matches = 0;
        for rle in resources.iter() {
            let mut img = Vec::<u8>::new();
            for ref pixel in &rle.image {
                img.push(pixel.r);
                img.push(pixel.g);
                img.push(pixel.b);
                img.push(pixel.a);
            }
            if let Some(file_num) = rle.file_num {
                for item in &list.items {
                    if item.file_number == file_num
                        && item.index == rle.index
                        {
                            matches += 1;
                            let file_name = format!("{}_{}.png",
                                                    &short_kind,
                                                    item.id);
                            let ent = RleCombiEntry {
                                id: item.id,
                                name: item.name.clone(),
                                x_offset: rle.offset_x,
                                y_offset: rle.offset_y,
                                width: rle.width,
                                height: rle.height,
                                file_name: file_name.clone(),
                            };
                            combi_entries.push(ent);

                            // Generate the png files
                            {
                                let mut path_buf = PathBuf::new();
                                path_buf.push(OUTPUT_PATH);
                                path_buf.push(&short_kind);
                                path_buf.push(file_name);
                                println!("{:?}", &path_buf);
                                let file = File::create(&path_buf).unwrap();
                                let ref mut writer = BufWriter::new(file);

                                let mut encoder = png::Encoder::new(writer,
                                                                    rle.width,
                                                                    rle.height);
                                encoder.set(png::ColorType::RGBA)
                                    .set(png::BitDepth::Eight);
                                let mut writer = encoder.write_header().unwrap();

                                writer.write_image_data(&img).unwrap();
                            }
                        }
                }
            }
        } // end resource iter

        // write out descriptor file
        {
            let file_name = format!("{}.xml", kind);
            let mut path_buf = PathBuf::new();
            path_buf.push(OUTPUT_PATH);
            path_buf.push(file_name);

            let file = File::create(&path_buf).unwrap();
            let writer = BufWriter::new(file);

            let kind_str = format!("{}", kind);
            {
                let mut xml = xml_writer::XmlWriter::new(writer);
                xml.begin_elem(&kind_str).unwrap();
                for entry in combi_entries {
                    xml.begin_elem("entry").unwrap();
                    xml.attr("id", &format!("{}", entry.id)).unwrap();
                    xml.attr("name", &entry.name).unwrap();
                    xml.attr("x_offset", &format!("{}", entry.x_offset)).unwrap();
                    xml.attr("y_offset", &format!("{}", entry.y_offset)).unwrap();
                    xml.attr("width", &format!("{}", entry.width)).unwrap();
                    xml.attr("height", &format!("{}", entry.height)).unwrap();
                    xml.attr("file_name", &entry.file_name).unwrap();
                    xml.end_elem().unwrap();
                }
                xml.end_elem().unwrap();
                xml.close().unwrap();
                xml.flush().unwrap();
            }
        }

        println!("resources.len()  == {:?}", &resources.len());
        println!("matches          == {:?}", matches);
    } // end kind entry loop
}

fn load_rmd_data(path: &Path, kind: RmdType) -> Result<Rmd, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;
    parse_rmd(kind, &bytes)
}

fn load_rmm_data(path: &Path) -> Result<Map, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;
    parse_rmm(&bytes)
}

fn load_list_data(path: &Path, use_v2: bool) -> Result<List, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;
    parse_lst(&bytes, use_v2)
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
            // we really only need a maximum of 5 digits...
            file_num = file_num % 99_999;
        }
    }

    // parse && append results
    parse_rle(file_num, &mut bytes)
}

struct RleCombiEntry {
    id: u32,
    name: String,
    x_offset: u32,
    y_offset: u32,
    width: u32,
    height: u32,
    file_name: String,
}

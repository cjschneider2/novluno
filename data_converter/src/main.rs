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

use core_compat::rle::{ResourceFile, Resource};
use core_compat::lst::List;
use core_compat::rmd::{Rmd, RmdType};
use core_compat::rmm::Map;
use core_compat::error::Error;

static OUTPUT_PATH: &'static str = "/home/schneider/temp/";

// This is the list of data folder's and list files for them
static RLE_ENTRIES: [(&'static str, &'static str, &'static str,
                      &'static str, bool); 16] = [
    ("bullets",   "bul", "../data/RLEs/Bul", "../data/RLEs/bul.lst", false),
    ("icons",     "ico", "../data/RLEs/Ico", "../data/RLEs/ico.lst", false),
    ("objects",   "obj", "../data/RLEs/Obj", "../data/RLEs/obj.lst", true ),
    ("tiles",     "tle", "../data/RLEs/Tle", "../data/RLEs/tle.lst", false),
    ("interface", "int", "../data/RLEs/Int", "../data/RLEs/int.lst", false),
    ("philar",    "ch0", "../data/RLEs/Chr/C00", "../data/RLEs/Chr/c00.lst", false),
    ("azlar",     "ch1", "../data/RLEs/Chr/C01", "../data/RLEs/Chr/c01.lst", false),
    ("sadad",     "ch2", "../data/RLEs/Chr/C02", "../data/RLEs/Chr/c02.lst", false),
    ("destino",   "ch3", "../data/RLEs/Chr/C03", "../data/RLEs/Chr/c03.lst", false),
    ("jarexx",    "ch4", "../data/RLEs/Chr/C04", "../data/RLEs/Chr/c04.lst", false),
    ("canon",     "ch5", "../data/RLEs/Chr/C05", "../data/RLEs/Chr/c05.lst", false),
    ("kitara",    "ch6", "../data/RLEs/Chr/C06", "../data/RLEs/Chr/c06.lst", false),
    ("lunarena",  "ch7", "../data/RLEs/Chr/C07", "../data/RLEs/Chr/c07.lst", false),
    ("lavita",    "ch8", "../data/RLEs/Chr/C08", "../data/RLEs/Chr/c08.lst", false),
    ("ch_9_gm",   "ch9", "../data/RLEs/Chr/C09", "../data/RLEs/Chr/c09.lst", false),
    ("extra_chr", "etc", "../data/RLEs/Chr/Etc", "../data/RLEs/Chr/etc.lst", false),
    // The sounds one is the only one which is a little different...
    // ("Sounds", "snd", "../data/RLEs/Snd", "../data/RLEs/snd.lst"),
];

static RMM_ENTRY: (&'static str, &'static str) =
    ("maps", "../data/DATAs/Map");

static RMD_ENTRIES: [(&'static str, &'static str, &'static str); 4] = [
    ("bullets", "bul", "../data/DATAs/Bul"),
    ("character", "bul", "../data/DATAs/Bul"),
    ("", "bul", "../data/DATAs/Bul"),
    ("", "bul", "../data/DATAs/Bul"),
];

fn main() {

    // parse the list file and insert them into the database
    convert_rle_data();

    // convert the maps ...
    // convert_rmm_data();

    // ... and rmd files
    // convert_rmd_data();

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
    Rmd::load(kind, &bytes)
}

fn load_rmm_data(path: &Path) -> Result<Map, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;
    Map::load(&bytes)
}

fn load_list_data(path: &Path, use_v2: bool) -> Result<List, Error> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::<u8>::new();
    file.read_to_end(&mut bytes)?;
    List::load(&bytes, use_v2)
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
    ResourceFile::load(file_num, &mut bytes)
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

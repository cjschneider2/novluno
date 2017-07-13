//! RMD files are sort of data pointers for the Objects and 
//! Tiles and include things like which file they are in and
//! if they are a part of an animation or so.
//!
//! [HEADER]
//! String
//! 12 empty bytes
//! String
//! int animation parts (this is for object animations)
//! int animation rows
//! String
//! int rmd rows
//! 
//! [RMD Row]
//! int ImageCount (How many images this row contains,
//!                 like an image from sadad for example can
//!                 contain: Body, color hair, color body, weapon)
//! 
//! [RMD Row - Images]
//! int SourceX
//! int SourceY
//! int SourceWidth
//! int SourceHeight
//! int empty
//! int renderz
//! int DestX
//! int DestY
//! int Draw Type (Shadow, skill, normal)
//! int ImageIDCount
//! 
//! [RMD Row - Images - Image ID]
//! int ImageID (Lst row pointer,
//!              this is a array so different weapons can be used)
//! 
//! int AnimationsCount
//! 
//! [RMD Animation]
//! int AnimationFrames
//! 
//! [RMD Animation - Frame]
//! int RMDRowPointer (points to a row of the RMD)

use std::str::from_utf8;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use error::Error;

#[derive(Debug)]
pub struct RmdImage {
    source_x: i32,
    source_y: i32,
    source_width: i32,
    source_height: i32,
    empty_1: u32,
    empty_2: u32,
    render_z: i32,
    dest_x: i32,
    dest_y: i32,
    draw_type: i32, // enum { Shadow, skill, normal }
    image_id_count: i32,
    image_id: Vec<i32> // Lst row pointer
}

impl RmdImage {
    fn new() -> RmdImage {
        RmdImage {
            source_x: 0,
            source_y: 0,
            source_width: 0,
            source_height: 0,
            empty_1: 0,
            empty_2: 0,
            render_z: 0,
            dest_x: 0,
            dest_y: 0,
            draw_type: 0,
            image_id_count: 0,
            image_id: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct RmdAnimation {
    frame_count: i32,
    frames: Vec<i16> // Rmd row pointer
}

#[derive(Debug)]
pub struct RmdRow {
    image_count: i32,
    images: Vec<RmdImage>,
    animation_count: i32,
    animations: Vec<RmdAnimation>,
}

impl RmdRow {
    fn new() -> RmdRow {
        RmdRow {
            image_count: 0,
            images: Vec::new(),
            animation_count: 0,
            animations: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Rmd {
    string: String,
    animation_parts: i32, // for object animaitons
    animation_rows: i32,
    row_count: i32,
    rows: Vec<RmdRow>,
    animation_count: i32,
    animations: Vec<RmdAnimation>,
}

impl Rmd {
    pub fn new() -> Rmd {
        Rmd {
            string: String::new(),
            animation_parts: 0,
            animation_rows: 0,
            row_count: 0,
            rows: Vec::new(),
            animation_count: 0,
            animations: Vec::new(),
        }
    }

    pub fn load(data: &[u8]) -> Result<Rmd, Error> {

        // This is just because tests always have a tab on the first line :-/
        println!("");

        let mut cursor = Cursor::new(data);
        let mut rmd = Rmd::new();

        // filetype string: Equal to ""
        let string_1 = parse_string(&mut cursor)?;
        println!("{:?}", string_1);

        let file_number = cursor.read_u32::<LE>()?; // 4
        println!("file_number: {}", file_number);

        // 8 empty bytes
        let padding = cursor.read_u32::<LE>()?; // 8
        if padding != 0 { println!("p2: {}", padding); }
        let padding = cursor.read_u32::<LE>()?; // 12
        if padding != 0 { println!("p3: {}", padding); }

        // let string = parse_string(&mut cursor)?;
        let string = parse_u8_vec(&mut cursor)?;
        println!("str 1: `{:?}`", string);

        rmd.animation_parts = cursor.read_i32::<LE>()?;
        rmd.animation_rows = cursor.read_i32::<LE>()?;

        let string = parse_string(&mut cursor)?;
        println!("str 2: `{}`", string);

        rmd.row_count = cursor.read_i32::<LE>()?;

        // read the Rmd rows
        for _ in 0..rmd.row_count {
            let mut row = RmdRow::new();
            row.image_count = cursor.read_i32::<LE>()?;
            for _ in 0..row.image_count {
                let mut img = RmdImage::new();
                img.source_x = cursor.read_i32::<LE>()?;
                img.source_y = cursor.read_i32::<LE>()?;
                img.source_width = cursor.read_i32::<LE>()?;
                img.source_height = cursor.read_i32::<LE>()?;
                img.empty_1 = cursor.read_u32::<LE>()?;
                img.dest_x = cursor.read_i32::<LE>()?;
                img.dest_y = cursor.read_i32::<LE>()?;
                img.empty_2 = cursor.read_u32::<LE>()?;
                img.render_z = cursor.read_i32::<LE>()?;
                img.draw_type = cursor.read_i32::<LE>()?;
                img.image_id_count = cursor.read_i32::<LE>()?;
                for _ in 0..img.image_id_count {
                    let id = cursor.read_i32::<LE>()?;
                    img.image_id.push(id);
                }
            }
            rmd.rows.push(row);
        }

        rmd.animation_count = cursor.read_i32::<LE>()?;

        for _ in 0..rmd.animation_count {
            let mut ani = RmdAnimation {
                frame_count: cursor.read_i32::<LE>()?,
                frames: Vec::new(),
            };
            for _ in 0..ani.frame_count {
                let ptr = cursor.read_i16::<LE>()?;
                ani.frames.push(ptr);
            }
            rmd.animations.push(ani);
        }

        // println!("{:?}", rmd);
        Ok(rmd)
    }
}

fn parse_string(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let string_length = cursor.read_u8()?;
    let mut str_vec = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        str_vec.push(chr);
    }
    let string = String::from_utf8(str_vec)?;
    Ok(string)
}

fn parse_u8_vec(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, Error> {
    let string_length = cursor.read_u8()?;
    let mut vec = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        vec.push(chr);
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tle_00001() {
        let data = include_bytes!("../../data/DATAs/Tle/tle00001.rmd");
        let rmd = Rmd::load(data).unwrap();
        assert!(rmd.row_count as usize == rmd.rows.len());
        assert!(rmd.animation_count as usize == rmd.animations.len());
    }

    #[test]
    fn test_obj_00001() {
        let data = include_bytes!("../../data/DATAs/Obj/obj00001.rmd");
        let rmd = Rmd::load(data).unwrap();
        assert!(rmd.row_count as usize == rmd.rows.len());
        assert!(rmd.animation_count as usize == rmd.animations.len());
    }

    #[test]
    fn test_chr_00001() {
        let data = include_bytes!("../../data/DATAs/Chr/chr00001.rmd");
        let rmd = Rmd::load(data).unwrap();
        assert!(rmd.row_count as usize == rmd.rows.len());
        assert!(rmd.animation_count as usize == rmd.animations.len());
    }

    #[test]
    fn test_chr_00042() {
        let data = include_bytes!("../../data/DATAs/Chr/chr00042.rmd");
        let rmd = Rmd::load(data).unwrap();
        assert!(rmd.row_count as usize == rmd.rows.len());
        assert!(rmd.animation_count as usize == rmd.animations.len());
    }
}

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
//! [RMD RowEntry]
//! int ImageCount (How many images this row/entry contains,
//!                 like an image from sadad for example can
//!                 contain: Body, color hair, color body, weapon)
//!
//! [RMD RowEntry - Images]
//! int SourceX      // upper-left  x coordinate
//! int SourceY      // upper-right x coordinate
//! int SourceWidth  // lower-left  y coordinate
//! int SourceHeight // lower-right y coordinate
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

use entity::rmd::Rmd;
use entity::rmd_animation::RmdAnimation;
use entity::rmd_entry::RmdEntry;
use entity::rmd_type::RmdType;
use entity::rmd_image::RmdImage;

use error::Error;

use utility::parsing::{parse_string, parse_cp949, parse_u8_vec};

pub fn parse_rmd(kind: RmdType, data: &[u8]) -> Result<Rmd, Error> {
    let mut cursor = Cursor::new(data);
    let mut rmd = Rmd::new(kind);

    // filetype string: Equal to ""
    let string_1 = parse_string(&mut cursor)?;
    // println!("{:?}", string_1);

    let file_number = cursor.read_u32::<LE>()?; // 4
    // println!("file_number: {}", file_number);

    // 8 empty bytes
    let padding = cursor.read_u32::<LE>()?; // 8
    if padding != 0 { println!("p2: {}", padding); }
    let padding = cursor.read_u32::<LE>()?; // 12
    if padding != 0 { println!("p3: {}", padding); }

    // let string = parse_string(&mut cursor)?;
    // let string = parse_u8_vec(&mut cursor)?;
    let string = parse_cp949(&mut cursor)?;
    // println!("str 1: `{:?}`", string);

    rmd.set_animation_parts(cursor.read_i32::<LE>()?);
    rmd.set_animation_entry_count(cursor.read_i32::<LE>()?);

    // let string = parse_u8_vec(&mut cursor)?;
    let string = parse_cp949(&mut cursor)?;
    // println!("str 2: `{:?}`", string);

    rmd.set_entry_count(cursor.read_i32::<LE>()?);

    // println!("end header offset: `{}`", cursor.position());

    // read the Rmd rows
    for _ in 0..rmd.entry_count() {
        let mut entry = RmdEntry::new();
        entry.set_image_count(cursor.read_i32::<LE>()?);
        for _ in 0..entry.image_count() {
            let mut img = RmdImage::new();
            img.source_x1 = cursor.read_i32::<LE>()?;
            img.source_y1 = cursor.read_i32::<LE>()?;
            img.source_x2 = cursor.read_i32::<LE>()?;
            img.source_y2 = cursor.read_i32::<LE>()?;
            img.empty_1   = cursor.read_i32::<LE>()?;
            img.empty_2   = cursor.read_i32::<LE>()?;
            img.dest_x    = cursor.read_i32::<LE>()?;
            img.dest_y    = cursor.read_i32::<LE>()?;
            img.render_z  = cursor.read_i32::<LE>()?;
            img.draw_type = cursor.read_i32::<LE>()?;
            img.image_id_count = cursor.read_i32::<LE>()?;
            for _ in 0..img.image_id_count {
                img.image_id.push(cursor.read_i32::<LE>()?);
            }
            entry.add_image(img);
        }
        rmd.add_entry(entry);
    }

    rmd.set_animation_count(cursor.read_i32::<LE>()?);

    for _ in 0..rmd.animation_count() {
        let mut ani = RmdAnimation::new(cursor.read_i32::<LE>()?);
        for _ in 0..ani.frame_count() {
            ani.add_frame(cursor.read_i16::<LE>()?);
        }
        rmd.add_animation(ani);
    }

    // println!("{:?}", rmd);
    Ok(rmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_ani_info(rmd: &Rmd) {
        println!();
        println!("rmd.animation_count: {:?}", &rmd.animation_count());
        println!("rmd.image_count:     {:?}", &rmd.entry_count());
        assert!(false);
    }

    #[test]
    fn test_tle_00001() {
        let data = include_bytes!("../../../data/DATAs/Tle/tle00001.rmd");
        let rmd = parse_rmd(RmdType::Tile, data).unwrap();
        // assert!(rmd.row_count() as usize == rmd.rows.len());
        // assert!(rmd.animation_count() as usize == rmd.animations.len());
    }

    #[test]
    fn test_obj_00001() {
        let data = include_bytes!("../../../data/DATAs/Obj/obj00001.rmd");
        let rmd = parse_rmd(RmdType::Object, data).unwrap();
        // assert!(rmd.row_count as usize == rmd.rows.len());
        // assert!(rmd.animation_count as usize == rmd.animations.len());
    }

    #[test]
    fn test_chr_00001() {
        let data = include_bytes!("../../../data/DATAs/Chr/chr00001.rmd");
        let rmd = parse_rmd(RmdType::Character, data).unwrap();
        // print_ani_info(&rmd);
        // assert!(rmd.row_count as usize == rmd.rows.len());
        // assert!(rmd.animation_count as usize == rmd.animations.len());
    }

    #[test]
    fn test_chr_00042() {
        let data = include_bytes!("../../../data/DATAs/Chr/chr00042.rmd");
        let rmd = parse_rmd(RmdType::Character, data).unwrap();
        // assert!(rmd.row_count as usize == rmd.rows.len());
        // assert!(rmd.animation_count as usize == rmd.animations.len());
    }
}

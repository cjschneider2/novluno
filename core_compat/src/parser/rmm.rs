//! The RMM map files have the following layout
//!
//! [HEADER]
//! String (first byte indicates how long the string is)
//! int map size x
//! int map size y
//! String (first byte indicates how long the string is)
//! int map number
//! int number of events
//!
//! [Event list (Number of events)] = Event clickable like mailbox
//! short eventnumber
//! int corner X1
//! int corner Y1
//! int corner X2
//! int corner Y2
//!
//! [XY List]
//! Here a few numbers are defined: Object = a tree, a house. Tle = floor tiles
//! Object number (RMD pointer)
//! Object part (RMD part pointer in the RMD file)
//! Tle number (RMD pointer)
//! Tle part (RMD part pointer in the RMD file)
//! Mouse indicator (Only used for mouseover on warp events 0 = nothing, 16 = warp event)
//! Collision (1XY has 2 spots to stand on and 4 different collision settings.
//!            No collision, full collision,
//!            left top collision, right bottom collision)


use std::str::from_utf8;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use crate::error::Error;
use crate::entity::map::Map;
use crate::entity::map_tile::MapTile;
use crate::entity::event::Event;
use crate::entity::entry::Entry;

pub fn parse_rmm(data: &[u8]) -> Result<Map, Error> {
    let mut cursor = Cursor::new(data);
    let mut map = Map::new();

    // filetype string: needs to equal "Redmoon MapData 1.0"
    let string_length = cursor.read_u8()?;
    let mut string = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        string.push(chr);
    }
    let file_type: &str = from_utf8(&string)?;

    if file_type != "RedMoon MapData 1.0" {
        // println!("{:?}", file_type);
        return Err(Error::MissingMapIdentifier);
    }

    // map size (x, y) in number of tiles
    map.set_size_x(cursor.read_u32::<LE>()?);
    map.set_size_y(cursor.read_u32::<LE>()?);

    // Map name?
    map.set_id_count(cursor.read_u8()?);
    for idx in 0..(map.id_count()) {
        map.add_id_list_val(cursor.read_u8()?);
    }
    map.name = cp949::cp949_to_utf8(&map.id_list);

    // the map number described by this file...
    map.set_map_number(cursor.read_u32::<LE>()?);
    map.set_event_count(cursor.read_u32::<LE>()?);

    // NOTE: This is an array of event rectangles for interactions with
    //       things like mailboxes and the like
    for _ in 0..map.event_count() {
        let event = Event {
            number: cursor.read_u16::<LE>()?,
            left: cursor.read_u32::<LE>()?,
            top: cursor.read_u32::<LE>()?,
            right: cursor.read_u32::<LE>()?,
            bottom: cursor.read_u32::<LE>()?,
        };
        if event.number != 0 {
            map.add_event(event);
        }
    }

    // read in the tile values...
    let count = map.size_x() * map.size_y();
    for tile in 0..count {
        let tile = parse_v1(&mut cursor)?;
        map.add_tile(tile);
    }

    Ok(map)
}

fn parse_v1(cursor: &mut Cursor<&[u8]>) -> Result<MapTile, Error> {
    let b_0: u32 = cursor.read_u8()? as u32;
    let b_1: u32 = cursor.read_u8()? as u32;
    let b_2: u32 = cursor.read_u8()? as u32;
    let b_3: u32 = cursor.read_u8()? as u32;
    let b_4: u32 = cursor.read_u8()? as u32;
    let b_5: u32 = cursor.read_u8()? as u32;
    let b_6: u32 = cursor.read_u8()? as u32;
    let b_7: u32 = cursor.read_u8()? as u32;

    assert_eq!(b_0 & 0x2, 0);

    let obj_file_num = (b_0 / 4) + (b_1 % 32) * 64;
    let tle_file_idx = ((b_2 % 128) * 8) + (b_1 / 32);
    let tle_file_num = (b_3 * 2) + (b_2 / 128);
    let warp = b_4;
    let collision = b_6;
    let obj_file_idx = if collision % 24 == 0 {
        b_7 << 1
    } else {
        (b_7 << 1) + 1
    };

    let tile = MapTile {
        obj_rmd_entry: Entry::new(obj_file_num, obj_file_idx),
        tle_rmd_entry: Entry::new(tle_file_num, tle_file_idx),
        warp,
        collision,
    };

    Ok(tile)
}

// NOTE: This was a test to see if pulling out the bit fields could be made a little better
fn parse_v2(cursor: &mut Cursor<&[u8]>) -> Result<MapTile, Error> {
    let b_0: u32 = cursor.read_u8()? as u32;
    let b_1: u32 = cursor.read_u8()? as u32;
    let b_2: u32 = cursor.read_u8()? as u32;
    let b_3: u32 = cursor.read_u8()? as u32;
    let b_4: u32 = cursor.read_u8()? as u32;
    let b_5: u32 = cursor.read_u8()? as u32;
    let b_6: u32 = cursor.read_u8()? as u32;
    let b_7: u32 = cursor.read_u8()? as u32;

    assert_eq!(b_0 & 0x2, 0);

    let obj_file_num = (b_0 >> 2) + ((b_1 & 0x1F) << 6);
    let tle_file_idx = (b_1 >> 5) + ((b_2 & 0x7F) << 3);
    let tle_file_num = (b_2 >> 7) + (b_3 << 1);
    let warp = b_4;
    // let collision    = (b_6 & 0xF0) >> 4;
    let collision = b_6;
    let obj_file_idx = if b_6 % 24 == 0 {
        b_7 << 1
    } else {
        (b_7 << 1) + 1
    };

    let tile = MapTile {
        obj_rmd_entry: Entry::new(obj_file_num, obj_file_idx),
        tle_rmd_entry: Entry::new(tle_file_num, tle_file_idx),
        warp,
        collision,
    };

    Ok(tile)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map00000_rmm() {
        let data = include_bytes!("../../../data/DATAs/Map/Map00000.rmm");
        let map = parse_rmm(data).unwrap();
        assert_eq!((map.size_x() * map.size_y()) as usize, map.tile_count());
    }

    #[test]
    fn test_map00001_rmm() {
        let data = include_bytes!("../../../data/DATAs/Map/Map00001.rmm");
        let map = parse_rmm(data).unwrap();
        assert_eq!((map.size_x() * map.size_y()) as usize, map.tile_count());
    }

    #[test]
    fn test_map00005_rmm() {
        let data = include_bytes!("../../../data/DATAs/Map/Map00005.rmm");
        let map = parse_rmm(data).unwrap();
        assert_eq!((map.size_x() * map.size_y()) as usize, map.tile_count());
    }

    #[test]
    fn test_map00003_rmm() {
        let data = include_bytes!("../../../data/DATAs/Map/Map00003.rmm");
        let map = parse_rmm(data).unwrap();
        assert_eq!((map.size_x() * map.size_y()) as usize, map.tile_count());
    }
}

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

use error::Error;

#[derive(Debug)]
pub struct Event {
    pub number: u16,
    pub c1_x: u32,
    pub c1_y: u32,
    pub c2_x: u32,
    pub c2_y: u32,
}

#[derive(Debug)]
pub struct Map {
    pub size_x: u32,
    pub size_y: u32,
    pub id_count: u8,
    pub id_list: Vec<u8>,
    pub number: u32,
    pub event_count: u32,
    pub events: Vec<Event>,
    pub tiles: Vec<MapTile>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MapTile {
    pub object_file_num: u32, // points to a `/DATAs/Obj/*.rmd
    pub object_file_idx: u32, // this is an index into this file
    pub tile_file_num: u32,   // points to a `/DATAs/Tle/*.rmd
    pub tile_file_idx: u32,   // this is an index into this file
    pub warp: u32,
    pub collision: u32,
}

impl Map {
    pub fn new() -> Map {
        Map {
            size_x: 0,
            size_y: 0,
            id_count: 0,
            id_list: Vec::new(),
            number: 0,
            event_count: 0,
            events: Vec::new(),
            tiles: Vec::new(),
        }
    }

    pub fn load(data: &[u8]) -> Result<Map, Error> {

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
            println!("{:?}", file_type);
            return Err(Error::MissingMapIdentifier);
        }

        // map size (x, y) in number of tiles
        map.size_x = cursor.read_u32::<LE>()?;
        map.size_y = cursor.read_u32::<LE>()?;

        // Map String (name?)
        map.id_count = cursor.read_u8()?;
        for idx in 0..(map.id_count) {
            let val = cursor.read_u8()?;
            map.id_list.push(val);
        }

        // the map number described by this file...
        map.number = cursor.read_u32::<LE>()?;
        map.event_count = cursor.read_u32::<LE>()?;

        // NOTE: This is an array of event rectangles for interactions with
        //       things like mailboxes and the like
        for _ in 0..map.event_count {
            let event = Event {
                number: cursor.read_u16::<LE>()?,
                c1_x: cursor.read_u32::<LE>()?,
                c1_y: cursor.read_u32::<LE>()?,
                c2_x: cursor.read_u32::<LE>()?,
                c2_y: cursor.read_u32::<LE>()?,
            };
            map.events.push(event);
        }

        // read in the tile values...
        let count = map.size_x * map.size_y;
        for tile in 0..count {
            let tile = parse_v1(&mut cursor)?;
            map.tiles.push(tile);
        }

        Ok(map)
    }
}

fn parse_v1 (cursor: &mut Cursor<&[u8]>) -> Result<MapTile, Error> {
    let b_0: u32 = cursor.read_u8()? as u32;
    let b_1: u32 = cursor.read_u8()? as u32;
    let b_2: u32 = cursor.read_u8()? as u32;
    let b_3: u32 = cursor.read_u8()? as u32;
    let b_4: u32 = cursor.read_u8()? as u32;
    let b_5: u32 = cursor.read_u8()? as u32;
    let b_6: u32 = cursor.read_u8()? as u32;
    let b_7: u32 = cursor.read_u8()? as u32;

    assert!(b_0 & 0x2 == 0);

    let obj_file_num = (b_0 / 4) + (b_1 % 32) * 64;
    let tle_file_idx = ((b_2 % 128) * 8) + (b_1 / 32);
    let tle_file_num = (b_3 * 2) + (b_2 / 128);
    let warp         = b_4;
    let collision    = b_6;
    let obj_file_idx = if collision % 24 == 0 {
                           (b_7 << 1)
                       } else {
                           (b_7 << 1) + 1
                       };

    let tile = MapTile {
        object_file_num: obj_file_num,
        object_file_idx: obj_file_idx,
        tile_file_num: tle_file_num,
        tile_file_idx: tle_file_idx,
        warp: warp,
        collision: collision,
    };

    Ok(tile)
}

fn parse_v2 (cursor: &mut Cursor<&[u8]>) -> Result<MapTile, Error> {
    let b_0: u32 = cursor.read_u8()? as u32;
    let b_1: u32 = cursor.read_u8()? as u32;
    let b_2: u32 = cursor.read_u8()? as u32;
    let b_3: u32 = cursor.read_u8()? as u32;
    let b_4: u32 = cursor.read_u8()? as u32;
    let b_5: u32 = cursor.read_u8()? as u32;
    let b_6: u32 = cursor.read_u8()? as u32;
    let b_7: u32 = cursor.read_u8()? as u32;

    assert!(b_0 & 0x2 == 0);

    let obj_file_num = (b_0 >> 2) + (( b_1 & 0x1F) << 6);
    let tle_file_idx = (b_1 >> 5) + ((b_2 & 0x7F) << 3);
    let tle_file_num = (b_2 >> 7) + (b_3 << 1);
    let warp         = b_4;
    // let collision    = (b_6 & 0xF0) >> 4;
    let collision    = b_6;
    let obj_file_idx = if b_6 % 24 == 0 {
                           (b_7 << 1)
                       } else {
                           (b_7 << 1) + 1
                       };

    let tile = MapTile {
        object_file_num: obj_file_num,
        object_file_idx: obj_file_idx,
        tile_file_num: tle_file_num,
        tile_file_idx: tle_file_idx,
        warp: warp,
        collision: collision,
    };

    Ok(tile)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map00001_rmm() {
        let data = include_bytes!("../../data/DATAs/Map/Map00001.rmm");
        let map = Map::load(data).unwrap();
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
    #[test]
    fn test_map00005_rmm() {
        let data = include_bytes!("../../data/DATAs/Map/Map00005.rmm");
        let map = Map::load(data).unwrap();
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
    #[test]
    fn test_map00003_rmm() {
        let data = include_bytes!("../../data/DATAs/Map/Map00003.rmm");
        let map = Map::load(data).unwrap();
        assert_eq!((map.size_x * map.size_y) as usize, map.tiles.len());
    }
}

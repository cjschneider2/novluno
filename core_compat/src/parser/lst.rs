use std::str::from_utf8;
use std::io::Cursor;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use error::Error;
use entity::list::List;
use entity::list_item::ListItem;

pub fn parse_lst(data: &[u8], use_v2: bool) -> Result<List, Error> {
    let mut cursor = Cursor::new(data);
    // filetype len prefixed string:
    //  - needs to equal "RedMoon Lst File"
    let string_length = cursor.read_u8()?;
    let mut string = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        string.push(chr);
    }
    {
        let file_type: &str = from_utf8(&string)?;
        // println!("{:?}", &file_type);
    }
    // file version length prefixed string
    let version: &str;
    {
        let version_length = cursor.read_u8()?;
        string.clear();
        for _ in 0..version_length {
            let chr = cursor.read_u8()?;
            string.push(chr);
        }
        version = from_utf8(&string)?;
        // println!("{:?}", &version);
    }

    if use_v2 {
        load_1_2(&mut cursor)
    } else {
        match version {
            "1.0" => load_1_0(&mut cursor),
            "1.2" => load_1_2(&mut cursor),
            _ => panic!("Unknown version type: {:?}", version),
        }
    }
}

/// The 1.0 format is used in most of the list files
fn load_1_0(cursor: &mut Cursor<&[u8]>) -> Result<List, Error> {
    let mut list = List::new();
    let mut string = Vec::<u8>::new();

    // Unknown u32
    let next_free_id = cursor.read_u32::<LE>()?;
    // list Entry counts
    let entry_count = cursor.read_u32::<LE>()?;
    // read entries
    for _ in 0..entry_count {
        // entry name
        let name_length = cursor.read_u8()?;
        string.clear();
        for _ in 0..name_length {
            let chr = cursor.read_u8()?;
            string.push(chr);
        }
        let name = from_utf8(&string).unwrap_or("wrong encoding?!").into();
        // rest of entry info
        let item = ListItem {
            name: name,
            id: cursor.read_u32::<LE>()?,
            file_number: cursor.read_u32::<LE>()?,
            index: cursor.read_u32::<LE>()?,
        };
        list.items.push(item);
    }
    Ok(list)
}

/// The 1.2 format seems to only be used in the `Obj` rle list file
fn load_1_2(cursor: &mut Cursor<&[u8]>) -> Result<List, Error> {
    let mut list = List::new();
    let mut string = Vec::<u8>::new();

    // Unknown u32
    let next_free_id = cursor.read_u32::<LE>()?;
    // list Entry counts
    let entry_count = cursor.read_u32::<LE>()?;
    // read entries
    for _ in 0..entry_count {
        // entry name
        let name_length = cursor.read_u8()?;
        string.clear();
        for _ in 0..name_length {
            let chr = cursor.read_u8()?;
            string.push(chr);
        }
        let name = from_utf8(&string).unwrap_or("wrong encoding?!").into();
        // I'm sort of assuming that we're trying to link to the "next id?"
        // here in the newer format?
        let id = cursor.read_u32::<LE>()?;
        let file_number = cursor.read_u32::<LE>()?;
        let index = cursor.read_u32::<LE>()?;
        let unknown_2 = cursor.read_u32::<LE>()?;

        // rest of entry info
        let item = ListItem {
            name: name,
            file_number: file_number,
            index: index,
            id: id,
        };
        list.items.push(item);
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lst_bul() {
        let data = include_bytes!("../../../data/RLEs/bul.lst");
        let list = parse_lst(data, false).unwrap();
    }

    #[test]
    fn test_lst_ico() {
        let data = include_bytes!("../../../data/RLEs/ico.lst");
        let list = parse_lst(data, false).unwrap();
    }

    #[test]
    fn test_lst_int() {
        let data = include_bytes!("../../../data/RLEs/int.lst");
        let list = parse_lst(data, false).unwrap();
    }

    #[test]
    fn test_lst_tle() {
        let data = include_bytes!("../../../data/RLEs/tle.lst");
        let list = parse_lst(data, false).unwrap();
    }

    // #[test]
    // // NOTE: This is the only one to fail on the 1.0 version
    // fn test_lst_snd() {
    //     let data = include_bytes!("../../data/RLEs/snd.lst");
    //     println!("data.len() = 0x{:X} bytes", data.len());
    //     let list = List::load(data, false);
    //     let list = match list {
    //         Ok(_) => list,
    //         Err(_) => {
    //             // maybe it really is in version 1.2 format?
    //             List::load(data, true)
    //         }
    //     };
    //     list.unwrap();
    // }

    #[test]
    // NOTE: This uses the version 1.2 of the lst file
    fn test_lst_obj() {
        let data = include_bytes!("../../../data/RLEs/obj.lst");
        let list = parse_lst(data, false).unwrap();
    }
}

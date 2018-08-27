use std::io::Cursor;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use cp949::cp949_to_utf8;

use error::Error;

pub fn parse_string(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let string_length = cursor.read_u8()?;
    let mut str_vec = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        str_vec.push(chr);
    }
    let string = String::from_utf8(str_vec)?;
    Ok(string)
}

pub fn parse_cp949(cursor: &mut Cursor<&[u8]>) -> Result<String, Error> {
    let string_length = cursor.read_u8()?;
    let mut str_vec = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        str_vec.push(chr);
    }
    let string = cp949_to_utf8(&str_vec);
    Ok(string)
}

pub fn parse_u8_vec(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, Error> {
    let string_length = cursor.read_u8()?;
    let mut vec = Vec::<u8>::new();
    for _ in 0..string_length {
        let chr = cursor.read_u8()?;
        vec.push(chr);
    }
    Ok(vec)
}

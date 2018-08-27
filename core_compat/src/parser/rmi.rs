//! "RedMoon EventInfo File 1.0"

use std::str::from_utf8;
use std::io::Cursor;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use entity::rmi::Rmi;

use error::Error;

use utility::parsing::{parse_string, parse_cp949, parse_u8_vec};

pub fn parse_rmi(data: &[u8]) -> Result<Rmi, Error> {
    let mut cursor = Cursor::new(data);
    let rmi = Rmi::new();

    // -- header
    let file_type_string = parse_string(&mut cursor).unwrap();
    println!("file_type_string: {:?}", file_type_string);

    let unknown_1 = cursor.read_i32::<LE>()?;
    println!("unknown_1: {:?}", unknown_1);

    let unknown_2 = cursor.read_i32::<LE>()?;
    println!("unknown_2: {:?}", unknown_2);

    let pad_1 = cursor.read_i16::<LE>()?;
    println!("pad_1: {:?}", pad_1);

    let unknown_3 = cursor.read_i32::<LE>()?;
    println!("unknown_3: {:?}", unknown_3);

    // -- entries
    for idx in 0..unknown_2 {
        println!("----------");
        println!("-- Cursor Start @ 0x{:x}", cursor.position());

        let e_unknown_1 = cursor.read_i32::<LE>()?;
        println!("e_unknown_1: {:?}", e_unknown_1);

        let e_string_1 = parse_cp949(&mut cursor).unwrap();

        println!("e_string_1: {:?}", e_string_1);
        let null_byte = cursor.read_u8()?;
        println!("null_byte: {:?}", null_byte);

        let e_string_2 = parse_cp949(&mut cursor).unwrap();
        println!("e_string_2: {:?}", e_string_2);

        assert_eq!(null_byte, 0);
    }

    Ok(rmi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmi_event00() {
        let data = include_bytes!("../../../data/DATAs/Info/event00.rmi");
        let rmi = parse_rmi(data);
        rmi.unwrap();
    }
}

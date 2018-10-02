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

    let count = cursor.read_i32::<LE>()?;
    println!("count?: {:?}", count);

    // -- entries
    for idx in 0..count {
        println!("----------");
        println!("-- Cursor Start @ 0x{:x}", cursor.position());

        let event_type = cursor.read_i32::<LE>()?;
        println!("event_type: {:?}", event_type);
        assert_eq!(68, event_type);

        let event_pad_1 = cursor.read_u8()?;
        println!("event_pad_1: {:?}", event_pad_1);
        let event_pad_2 = cursor.read_u8()?;
        println!("event_pad_2: {:?}", event_pad_2);

        let event_count = cursor.read_i32::<LE>()?;
        println!("event_count: {:?}", event_count);

        for e_idx in 0..event_count {
            println!("{{");
            println!("    idx: {:?}", e_idx);

            let action_timeout = cursor.read_i32::<LE>()?;
            println!("    action_timout: {:?}", action_timeout);

            let pos = cursor.position();
            let byte = cursor.read_u8()?;
            if byte != 0 { cursor.set_position(pos); }
            let trigger_string = parse_cp949(&mut cursor).unwrap();
            println!("    trigger_string: {:?}", trigger_string);


            let pos = cursor.position();
            let byte = cursor.read_u8()?;
            if byte != 0 { cursor.set_position(pos); }
            let action_string = parse_cp949(&mut cursor).unwrap();
            println!("    action_string: {:?}", action_string);

            println!("}}");
        }

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

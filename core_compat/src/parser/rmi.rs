//! "RedMoon EventInfo File 1.0"

use std::str::from_utf8;
use std::io::Cursor;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;

use crate::entity::rmi::Rmi;
use crate::error::Error;
use crate::utility::parsing::{parse_string, parse_cp949, parse_u8_vec};

const ITEM_INFO_HDR: &str = "RedMoon ItemInfo File 1.0";
const EVENT_INFO_HDR: &str = "RedMoon EventInfo File 1.0";

pub fn parse_rmi(data: &[u8]) -> Result<Rmi, Error> {
    let mut cursor = Cursor::new(data);
    let rmi = Rmi::new();

    // -- header
    let file_type_string = parse_string(&mut cursor).unwrap();
    println!("file_type_string: {:?}", file_type_string);

    let count = cursor.read_i32::<LE>()?;
    println!("count?: {:?}", count);

    for idx in 0..count {
        let _ = parse_event_entry(&mut cursor, idx).unwrap();
    }

    Ok(rmi)
}

fn parse_event_entry(
    cursor: &mut Cursor<&[u8]>,
    idx: i32)
    -> Result<(), Error>
{
    println!("-- Entry {}", idx);
    println!("-- Cursor Start @ 0x{:x}", cursor.position());

    let event_type = cursor.read_u16::<LE>()?;
    println!("event_type: {:?}", event_type);
    if event_type != 0x44 || event_type != 0x60EA {
        println!("unknown event type {:?}", event_type);
    }

    let event_unknown = cursor.read_i32::<LE>()?;
    println!("event_unknown: {:?}", event_unknown);

    let event_count = cursor.read_i32::<LE>()?;
    println!("event_count: {:?}", event_count);

    for e_idx in 0..event_count {
        println!("{{");
        println!("    idx: {:?}", e_idx);

        let action_timeout = cursor.read_i32::<LE>()?;
        println!("    action_timout: {:?}", action_timeout);

        let trigger_string = parse_cp949(cursor).unwrap();
        println!("    trigger_string: {:?}", trigger_string);

        let pos = cursor.position();
        let byte = cursor.read_u8()?;
        if byte != 0 {
            cursor.set_position(pos);
            println!("    -- byte value 0x{:x} @ 0x{:x}", byte, pos);
        }

        let mut cont = true;
        while cont {
            let action_string = parse_cp949(cursor).unwrap();
            println!("    action_string: {:?}", action_string);
            let pos = cursor.position();
            let byte = cursor.read_u8()?;
            println!("    -- byte value 0x{:x} @ 0x{:x}", byte, pos);
            if byte <= 1
            || byte == 0x44
            || byte == 0x60 {
                cont = false;
            }
            cursor.set_position(pos);
        }

        println!("}}");
    }

    Ok(())
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

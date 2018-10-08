extern crate byteorder;

pub mod character_table;

use std::io::Cursor;

use byteorder::ReadBytesExt;

use crate::character_table::CP949_TABLE;

const REPLACEMENT_CHARACTER: u32 = 0xFFFD;

fn lookup_949_char(input: u16) -> u32 {
    for entry in CP949_TABLE.iter() {
        if entry.cv == input {
            return entry.uv as u32
        }
    }
    REPLACEMENT_CHARACTER
}

pub fn cp949_to_utf8(input: &[u8]) -> String {

    let mut output = String::new();
    let mut cursor = Cursor::new(input);
    let input_len = input.len() as u64;

    while cursor.position() < input_len {
        let idx = cursor.position();
        let uni_code_point: u32 = match cursor.read_u8().unwrap() as u32 {
            val @ 0x00 ... 0x7F => val as u32,
            0x80 | 0xFF         => REPLACEMENT_CHARACTER, // undefined values
            val @ 0x81 ... 0xFE => {
                // lead byte encountered
                if idx + 1 < input_len {
                    let next = cursor.read_u8().unwrap() as u16;
                    let mut c: u16 = ((val as u16) << 8) + next;
                    lookup_949_char(c) as u32
                } else {
                    REPLACEMENT_CHARACTER
                }
            },
            _ => REPLACEMENT_CHARACTER
        };
        let c = std::char::from_u32(uni_code_point).unwrap();
        output.push(c);
    }

    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

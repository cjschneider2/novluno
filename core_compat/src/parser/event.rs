use std::io::Cursor;
use std::str::from_utf8;
use byteorder::ReadBytesExt;
use byteorder::LittleEndian as LE;
use utility::parsing::parse_string;
use crate::error::Error;
use crate::{parser, utility};
use crate::utility::parsing::{parse_cp949, parse_fixed_len_string};

#[derive(Debug, Clone)]
pub struct Event {
    event: String,
    data: String,
}

#[derive(Debug, Clone)]
pub enum EventType {
    AppearAt,
    ChatOneByCode,
    Timeout,
    Walk,
    Run,
    Turn,
    Move,
    BonusPoint,
    CloseWindow,
    BreakGroup,
    Key,
    CreateGroup,
    ChatAllByCode,
}

impl EventType {
    pub fn from_string(id: &str) -> Option<EventType> {
        match id {
            "Run" => Some(EventType::Run),
            "Walk" => Some(EventType::Walk),
            "Timeout" => Some(EventType::Timeout),
            "AppearAt" => Some(EventType::AppearAt),
            "ChatOneByCode" => Some(EventType::ChatOneByCode),
            id => panic!("unknown event id: {:?}", id)
        }
    }
}

pub fn parse_events(data: &[u8]) -> Result<Vec<Event>, Error> {
    let mut cursor = Cursor::new(data);
    let mut events = Vec::<Event>::new();

    let file_type = parse_string(&mut cursor)?;
    if file_type != "RedMoon EventInfo File 1.0" {
        panic!("{:?}", file_type);
    }

    let total_events = cursor.read_u32::<LE>()?;
    dbg!(&total_events);

    let unknown_1 = cursor.read_u32::<LE>()?;
    dbg!(&unknown_1);

    let padding = cursor.read_u16::<LE>()?;

    for _ in 0..total_events{
        let event_count = cursor.read_u32::<LE>()?;
        let unknown_5 = cursor.read_u32::<LE>()?;
        for _ in 0..event_count {
            let event_string = parse_cp949(&mut cursor)?;
            dbg!(&event_string);

            let event = Event {
                event: event_string,
                data: String::new(),
            };
            events.push(event);

        }
    }

    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event00001() {
        let data = include_bytes!("../../../data/DATAs/Info/event00.rmi");
        let events = parse_events(data).unwrap();
        dbg!(&events);
        assert_eq!(events.len(), 19);
    }
}
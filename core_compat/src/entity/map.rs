
use entity::event::Event;
use entity::map_tile::MapTile;

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
}

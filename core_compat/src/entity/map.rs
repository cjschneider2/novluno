
use crate::entity::event::Event;
use crate::entity::map_tile::MapTile;

#[derive(Debug)]
pub struct Map {
    pub size_x: u32,
    pub size_y: u32,
    pub id_count: u8,
    pub id_list: Vec<u8>,
    pub name: String,
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
            name: String::new(),
            number: 0,
            event_count: 0,
            events: Vec::new(),
            tiles: Vec::new(),
        }
    }

    pub fn add_tile(&mut self, tile: MapTile) {
        self.tiles.push(tile);
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_id_list_val(&mut self, val: u8) {
        self.id_list.push(val);
    }

    pub fn set_event_count(&mut self, event_count: u32) {
        self.event_count = event_count;
    }

    pub fn set_map_number(&mut self, number: u32) {
        self.number = number;
    }

    pub fn set_id_count(&mut self, val: u8) {
        self.id_count = val;
    }

    pub fn set_size_x(&mut self, val: u32) {
        self.size_x = val;
    }

    pub fn set_size_y(&mut self, val: u32) {
        self.size_y = val;
    }

    pub fn id_count(&self) -> u8 {
        self.id_count
    }

    pub fn event_count(&self) -> u32 {
        self.event_count
    }

    pub fn size_x(&self) -> u32 {
        self.size_x
    }

    pub fn size_y(&self) -> u32 {
        self.size_y
    }

    pub fn tiles(&self) -> &[MapTile] {
        &self.tiles
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_tile(&self, index: usize) -> Option<&MapTile> {
        self.tiles.get(index)
    }

    pub fn get_size_x(&self) -> u32 {
        self.size_x
    }

    pub fn get_size_y(&self) -> u32 {
        self.size_y
    }

    pub fn number(&self) -> u32 {
        self.number
    }
}

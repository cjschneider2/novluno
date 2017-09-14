use utility::pixel::Pixel;

#[derive(Debug)]
pub struct Resource {
    pub file_num: Option<u32>,
    pub index: u32,
    pub offset: u32,
    pub len: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub unknown_1: u32,
    pub unknown_2: u32,
    pub unknown_3: u32,
    pub unknown_4: u32,
    pub image: Vec<Pixel>,
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            file_num: None,
            index: 0,
            offset: 0,
            len: 0,
            offset_x: 0,
            offset_y: 0,
            width: 0,
            height: 0,
            unknown_1: 0,
            unknown_2: 0,
            unknown_3: 0,
            unknown_4: 0,
            image: Vec::new(),
        }
    }
}

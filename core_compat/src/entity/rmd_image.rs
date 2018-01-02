use utility::rectangle::Rectangle;
use utility::size::Size;
use utility::point::Point;

#[derive(Debug)]
pub struct RmdImage {
    pub source_x1: i32,
    pub source_y1: i32,
    pub source_x2: i32,
    pub source_y2: i32,
    pub empty_1: i32,
    pub empty_2: i32,
    pub render_z: i32,
    pub dest_x: i32,
    pub dest_y: i32,
    pub draw_type: i32, // enum { Shadow, skill, normal }
    pub image_id_count: i32,
    pub image_id: Vec<i32>    // Lst row/entry pointer entries
}

impl RmdImage {
    pub fn new() -> RmdImage {
        RmdImage {
            source_x1: 0,
            source_y1: 0,
            source_x2: 0,
            source_y2: 0,
            empty_1: 0,
            empty_2: 0,
            render_z: 0,
            dest_x: 0,
            dest_y: 0,
            draw_type: 0,
            image_id_count: 0,
            image_id: Vec::new(),
        }
    }
}


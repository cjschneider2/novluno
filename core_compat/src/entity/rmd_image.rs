use utility::rectangle::Rectangle;
use utility::size::Size;
use utility::point::Point;

#[derive(Debug)]
pub struct RmdImage {
    // source_x: i32,
    // source_y: i32,
    // source_width: i32,
    // source_height: i32,
    source_rect: Rectangle<i32>,
    empty_1: u32,
    empty_2: u32,
    render_z: i32,
    // dest_x: i32,
    // dest_y: i32,
    dest_point: Point<i32>,
    draw_type: i32,
    // enum { Shadow, skill, normal }
    image_id_count: i32,
    image_id: Vec<i32> // Lst row/entry pointer entries
}

impl RmdImage {
    pub fn new() -> RmdImage {
        let location = Point { x: 0, y: 0 };
        let size = Size { width: 0, height: 0 };
        let dest_point = Point { x: 0, y: 0 };
        RmdImage {
            // source_x: 0,
            // source_y: 0,
            // source_width: 0,
            // source_height: 0,
            source_rect: Rectangle { location, size },
            empty_1: 0,
            empty_2: 0,
            render_z: 0,
            // dest_x: 0,
            // dest_y: 0,
            dest_point,
            draw_type: 0,
            image_id_count: 0,
            image_id: Vec::new(),
        }
    }

    pub fn add_image_id(&mut self, id: i32) { self.image_id.push(id); }

    pub fn set_image_id_count(&mut self, count: i32) { self.image_id_count = count; }

    pub fn image_id_count(&self) -> i32 { self.image_id_count }

    pub fn get_image_id_list(&self) -> &[i32] { &self.image_id }

    pub fn set_draw_type(&mut self, val: i32) { self.draw_type = val; }

    pub fn set_render_z(&mut self, val: i32) { self.render_z = val; }

    pub fn set_empty_1(&mut self, val: u32) { self.empty_1 = val; }

    pub fn set_empty_2(&mut self, val: u32) { self.empty_2 = val; }

    pub fn set_dest_y(&mut self, val: i32) { self.dest_point.x = val; }

    pub fn set_dest_x(&mut self, val: i32) { self.dest_point.y = val; }

    pub fn dest_x(&self) -> i32 { self.dest_point.x }

    pub fn dest_y(&self) -> i32 { self.dest_point.y }

    pub fn set_source_x(&mut self, val: i32) { self.source_rect.location.x = val; }

    pub fn set_source_y(&mut self, val: i32) { self.source_rect.location.y = val; }

    pub fn set_source_height(&mut self, val: i32) { self.source_rect.size.height = val; }

    pub fn set_source_width(&mut self, val: i32) { self.source_rect.size.width = val; }

    pub fn source_x(&self) -> i32 { self.source_rect.location.x }

    pub fn source_y(&self) -> i32 { self.source_rect.location.y }

    pub fn source_width(&self) -> i32 { self.source_rect.size.width }

    pub fn source_height(&self) -> i32 { self.source_rect.size.height }
}


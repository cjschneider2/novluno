
#[derive(Debug)]
pub struct RmdImage {
    source_x: i32,
    source_y: i32,
    source_width: i32,
    source_height: i32,
    empty_1: u32,
    empty_2: u32,
    render_z: i32,
    dest_x: i32,
    dest_y: i32,
    draw_type: i32, // enum { Shadow, skill, normal }
    image_id_count: i32,
    image_id: Vec<i32> // Lst row pointer
}

impl RmdImage {
    pub fn new() -> RmdImage {
        RmdImage {
            source_x: 0,
            source_y: 0,
            source_width: 0,
            source_height: 0,
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

    pub fn add_image_id(&mut self, id: i32) {
        self.image_id.push(id);
    }

    pub fn set_image_id_count(&mut self, count: i32) {
        self.image_id_count = count;
    }

    pub fn image_id_count(&self) -> i32 {
        self.image_id_count
    }

    pub fn set_draw_type(&mut self, val: i32) {
        self.draw_type = val;
    }

    pub fn set_render_z(&mut self, val: i32) {
        self.render_z = val;
    }

    pub fn set_empty_1(&mut self, val: u32) {
        self.empty_1 = val;
    }

    pub fn set_empty_2(&mut self, val: u32) {
        self.empty_2 = val;
    }

    pub fn set_dest_y(&mut self, val: i32) {
        self.dest_y = val;
    }

    pub fn set_dest_x(&mut self, val: i32) {
        self.dest_x = val;
    }

    pub fn set_source_height(&mut self, val: i32) {
        self.source_height = val;
    }

    pub fn set_source_width(&mut self, val: i32) {
        self.source_width = val;
    }

    pub fn set_source_x(&mut self, val: i32) {
        self.source_x = val;
    }

    pub fn set_source_y(&mut self, val: i32) {
        self.source_y = val;
    }
}


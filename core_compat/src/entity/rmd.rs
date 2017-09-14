
use entity::rmd_type::RmdType;
use entity::rmd_animation::RmdAnimation;
use entity::rmd_row::RmdRow;

#[derive(Debug)]
pub struct Rmd {
    kind: RmdType,
    string: String,
    animation_parts: i32, // for object animations
    animation_rows: i32,
    row_count: i32,
    rows: Vec<RmdRow>,
    animation_count: i32,
    animations: Vec<RmdAnimation>,
}

impl Rmd {
    pub fn new(kind: RmdType) -> Rmd {
        Rmd {
            kind,
            string: String::new(),
            animation_parts: 0,
            animation_rows: 0,
            row_count: 0,
            rows: Vec::new(),
            animation_count: 0,
            animations: Vec::new(),
        }
    }

    pub fn add_animation(&mut self, ani: RmdAnimation) {
        self.animations.push(ani);
    }

    pub fn set_animation_count(&mut self, count: i32) {
        self.animation_count = count;
    }

    pub fn animation_count(&self) -> i32 {
        self.animation_count
    }

    pub fn set_animation_parts(&mut self, parts: i32) {
        self.animation_parts = parts;
    }

    pub fn set_animation_rows(&mut self, rows: i32) {
        self.animation_rows = rows;
    }

    pub fn add_row(&mut self, row: RmdRow) {
        self.rows.push(row);
    }

    pub fn set_row_count(&mut self, count: i32) {
        self.row_count = count;
    }

    pub fn row_count(&self) -> i32 {
        self.row_count
    }
}

use entity::rmd_animation::RmdAnimation;
use entity::rmd_image::RmdImage;

#[derive(Debug)]
pub struct RmdRow {
    image_count: i32,
    images: Vec<RmdImage>,
    animation_count: i32,
    animations: Vec<RmdAnimation>,
}

impl RmdRow {
    pub fn new() -> RmdRow {
        RmdRow {
            image_count: 0,
            images: Vec::new(),
            animation_count: 0,
            animations: Vec::new(),
        }
    }

    pub fn set_image_count(&mut self, count: i32) {
        self.image_count = count;
    }

    pub fn image_count(&self) -> i32 {
        self.image_count
    }
}


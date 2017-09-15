use entity::rmd_animation::RmdAnimation;
use entity::rmd_image::RmdImage;

#[derive(Debug)]
pub struct RmdEntry {
    image_count: i32,
    images: Vec<RmdImage>,
    animation_count: i32,
    animations: Vec<RmdAnimation>,
}

impl RmdEntry {
    pub fn new() -> RmdEntry {
        RmdEntry {
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

    pub fn get_image(&self, index: usize) -> Option<&RmdImage> {
        self.images.get(index)
    }

    pub fn images(&self) -> &[RmdImage] {
        &self.images
    }
}


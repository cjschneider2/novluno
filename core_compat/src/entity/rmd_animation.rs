
#[derive(Debug)]
pub struct RmdAnimation {
    frame_count: i32,
    frames: Vec<i16> // Rmd row pointer
}

impl RmdAnimation {

    pub fn new(frame_count: i32) -> RmdAnimation {
        RmdAnimation {
            frame_count,
            frames: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, ptr: i16) {
        self.frames.push(ptr);
    }

    pub fn frame_count(&self) -> i32 {
        self.frame_count
    }

    pub fn frames(&self) -> &[i16] {
        &self.frames
    }
}

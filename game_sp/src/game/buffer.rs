
pub struct Image {
    pub memory: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bytes_per_pixel: u32,
}

pub struct SoundOutput {
    pub samples: Box<[i16]>,
    pub samples_per_second: usize,
    pub sample_count: usize,
}

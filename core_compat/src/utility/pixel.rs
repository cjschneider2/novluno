
#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a, }
    }
    pub fn new_empty() -> Pixel {
        Pixel { r: 0, g: 0, b: 0, a: 0, }
    }
}
